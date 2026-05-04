# Plan Adaptado PeruCRIS + Pure para PJUPI

> **⚠️ NOTA**: Este documento es histórico y referencia rutas/estados de la arquitectura anterior. La estructura actual es Screaming Architecture (ver `src-tauri/src/*/`). Se conserva como referencia del plan de integración Pure.

## 1) Resultado de auditoria de viabilidad

Estado real del proyecto (verificado):

- Backend actual: dual (MongoDB primario + SQLite local/offline).
- La seleccion de backend vive en servicios via BackendStrategy.
- SQLite se inicializa siempre en runtime y se usa para outbox/sync.
- Tauri v2 esta centralizado en lib.rs (correcto para desktop/mobile entrypoint).
- Frontend real usa modulos en src/features/*, no la estructura antigua src/components/*.

Conclusion:

- El plan PeruCRIS/Pure es viable tecnicamente.
- No es viable ejecutarlo "tal cual" por 3 diferencias:
  1. La restriccion "MongoDB exclusivo inmediato" rompe la arquitectura activa si se aplica de golpe.
  2. Algunos archivos/rutas del plan no existen en el repo actual.
  3. Faltan contratos de tipos/IPC para exponer las nuevas entidades al frontend.

## 2) Adaptaciones obligatorias al plan original

### A. Fase previa obligatoria (nueva): Fase 0 - Transicion Mongo only segura

Objetivo:

- Mantener estabilidad funcional mientras se retira el modo SQLite primario.

Acciones:

1. Forzar solo MongoDB como backend primario permitido en config_validator.
2. Mantener SQLite solo para telemetria/sync legacy durante transicion (si aun se requiere).
3. Marcar como deprecated:
   - services/backend_strategy.rs
   - infrastructure/*_repo.rs de SQLite
4. Definir fecha de corte para eliminar ramas is_sqlite_primary() en servicios.

Criterio de salida:

- Ningun flujo funcional depende de sqlite primary.
- Aplicacion levanta solo con MongoDB como path operativo soportado.

### B. Ajuste de rutas frontend

Plan original propone rutas antiguas; en este repo deben mapearse a:

- Docentes detalle: src/features/docentes/components/DocenteDetailModal.tsx
- Proyectos: src/features/proyectos/components/ProyectoCreateModal.tsx y ProyectoEditModal.tsx
- Nuevo modulo grupos: src/features/grupos/GruposTab.tsx (nuevo feature real)
- Integraciones API: src/services/tauri/* + src/features/*/api.ts

### C. Ajuste de inicializacion Tauri

- Registrar nuevos comandos en src-tauri/src/lib.rs (invoke_handler).
- No cargar logica en src-tauri/src/main.rs (mantener main fino).

## 3) Plan ejecutable por bloques (adaptado)

### Regla funcional transversal (nueva)

- Cardinalidad oficial:
   - Proyecto -> Patentes: 0..N (opcional)
   - Proyecto -> Publicaciones: 0..N (opcional)
   - Proyecto -> Productos: 0..N (opcional)
- Implicacion: un proyecto puede no tener registros de estos tipos y sigue siendo valido.
- Cuando existan, deben asociarse por proyecto_id y listarse como colecciones hijas del proyecto.

## BLOQUE 1 - Dominio (Rust)

Objetivo:

- Crear entidades PeruCRIS y extender entidades existentes sin romper compatibilidad.

Cambios:

1. Crear:
   - src-tauri/src/domain/equipamiento.rs
   - src-tauri/src/domain/patente.rs
   - src-tauri/src/domain/publicacion.rs
   - src-tauri/src/domain/producto.rs
   - src-tauri/src/domain/grupo_investigacion.rs
   - src-tauri/src/domain/financiamiento.rs
2. Actualizar src-tauri/src/domain/mod.rs.
3. Extender:
   - docente.rs: scopus_id, grupo_investigacion_id con serde(default).
   - proyecto.rs: campo_ocde, programas_relacionados con serde(default).
4. Normalizar fechas: usar i64 epoch ms (consistente con dominio actual) en vez de String ISO donde aplique.
5. Ajustar relaciones para que patente/publicacion/producto soporten asociacion opcional a proyecto:
   - proyecto_id como Option<String> para permitir 0..N sin obligar carga inicial.
   - En publicacion, mantener docente_id obligatorio (origen Pure por investigador) y proyecto_id opcional para vinculo institucional.

Reglas de validacion de dominio:

- No exigir patente/publicacion/producto al crear o editar proyecto.
- Si una patente/publicacion/producto no tiene proyecto_id, puede existir como registro independiente del docente.
- Si incluye proyecto_id, este debe existir en proyectos.

Criterios de aceptacion:

- cargo check sin errores.
- serde backward compatible (lectura de documentos antiguos sin fallar).

## BLOQUE 2 - Infraestructura MongoDB

Objetivo:

- Soportar persistencia e indices de nuevas colecciones.

Cambios:

1. Extender src-tauri/src/infrastructure/mongo_repo.rs con nuevas colecciones.
2. Crear indices:
   - publicaciones.pure_uuid unique
   - publicaciones.proyecto_id (no unique)
   - patentes.numero_patente (segun reglas de negocio; unique opcional si aplica)
   - patentes.proyecto_id (no unique)
   - productos.proyecto_id (no unique)
   - equipamientos.proyecto_id
   - financiamientos.proyecto_id
   - grupos_investigacion.coordinador_id
3. Añadir funciones CRUD minimas por agregado nuevo.

Criterios de aceptacion:

- init_mongo ejecuta ensure_indexes sin fallos.
- operaciones create/get basicas por agregado funcionan.
- consultas por proyecto devuelven 0..N resultados en patentes/publicaciones/productos sin error cuando no hay registros.

## BLOQUE 3 - Pure API + Comandos IPC

Objetivo:

- Sincronizar publicaciones desde Pure por docente/scopus id.

Regla de origen de datos (nueva):

- El Scopus ID para sincronizacion debe salir del docente ya registrado en PJUPI (integracion previa RENACYT).
- Campo canonico recomendado: renacyt_scopus_author_id (evitar duplicar con un nuevo scopus_id si no es estrictamente necesario).
- El comando IPC no debe depender de un scopus_id ingresado manualmente por UI si el docente ya lo tiene persistido.

Cambios:

1. Validar dependencia actual:
   - reqwest y dotenvy ya existen en Cargo.toml.
2. Crear src-tauri/src/infrastructure/pure_client.rs:
   - cliente HTTP con headers Accept y api-key.
   - parser robusto para items[] y campos opcionales.
3. Crear src-tauri/src/commands/pure_cmd.rs:
   - sync_publicaciones_pure(docente_id, state)
   - upsert por pure_uuid con update_one + upsert(true)
4. Exponer comando en:
   - src-tauri/src/commands/mod.rs
   - src-tauri/src/lib.rs invoke_handler
5. Agregar clave PURE_API_KEY al flujo de runtime config (evitar expect panico; devolver error controlado).
6. Resolver scopus_id con este orden:
   - 1) docente.renacyt_scopus_author_id
   - 2) fallback opcional a override manual solo para casos excepcionales y auditables
   - 3) si no existe identificador, retornar error funcional claro

Criterios de aceptacion:

- Si falta PURE_API_KEY: error funcional claro, no panic.
- Sync idempotente: multiples ejecuciones no duplican publicaciones.
- Si el docente no tiene Scopus ID, el sistema responde con mensaje accionable y no intenta llamar a Pure.
- Si el docente tiene Scopus ID, la UI puede sincronizar sin pedir ese dato nuevamente.

## BLOQUE 4 - Servicios y capa de aplicacion

Objetivo:

- Mantener reglas de negocio en servicios, no en comandos.

Cambios:

1. Crear service dedicado (ejemplo: publicaciones_service.rs).
2. Desde pure_cmd llamar al service, no directo a mongo_repo.
3. Si se activa Mongo-only definitivo:
   - eliminar ramas is_sqlite_primary() en servicios impactados.

Criterios de aceptacion:

- comandos delgados + servicios con logica.
- permisos/roles aplicados via storage.rs segun patron actual.

## BLOQUE 5 - Frontend (React)

Objetivo:

- Exponer nueva funcionalidad PeruCRIS/Pure en UI actual.

Cambios:

1. Docentes:
   - Actualizar src/features/docentes/components/DocenteDetailModal.tsx
   - Pestana Publicaciones con accion "Sincronizar Pure".
   - Pestana Otros Productos con formulario manual.
2. Proyectos:
   - Actualizar ProyectoCreateModal.tsx y ProyectoEditModal.tsx
   - Secciones opcionales (form-array): Patentes derivadas, Publicaciones asociadas, Productos I+D+i, Equipamiento y Financiamiento.
   - Si no se agrega ningun item en esas secciones, el proyecto se guarda igual.
3. Grupos:
   - Crear src/features/grupos/GruposTab.tsx
   - Integrar tab y permisos en App.tsx + capa auth.
4. Contratos TS:
   - ampliar src/services/tauri/types.ts
   - crear wrappers IPC en src/services/tauri/*.ts
   - re-export en src/features/*/api.ts

Criterios de aceptacion:

- npm run build exitoso.
- UI invoca IPC sin errores de nombre de comando.
- formularios de proyecto aceptan 0 elementos y tambien multiples elementos por cada seccion relacionada.

## BLOQUE 6 - Seguridad y operacion

Objetivo:

- Evitar fuga de secretos y endurecer despliegue.

Cambios:

1. No persistir PURE_API_KEY en logs ni errores.
2. Incluir validacion de PURE_API_KEY en config_validator.rs (si feature Pure habilitada).
3. Actualizar setup_wizard para soportar API key opcional/rotable.
4. Documentar variables en README y guia tecnica.

Criterios de aceptacion:

- ningun log imprime credenciales.
- errores accionables para configuracion incompleta.

## 4) Riesgos principales y mitigacion

1. Ruptura por Mongo-only abrupto:
- Mitigar con Fase 0 y feature flag temporal.

2. Contrato Pure API variable:
- Mitigar con parser defensivo + pruebas con payload real.

3. Complejidad UI en una sola entrega:
- Mitigar con release incremental: Docentes (Pure) antes que Proyectos+Grupos.

4. Regresion de permisos:
- Mitigar agregando nuevas permissions en el mismo patron de storage.rs.

## 5) Orden recomendado de implementacion real

1. Fase 0 Mongo-only segura.
2. Bloque 1 Dominio.
3. Bloque 2 Infra Mongo.
4. Bloque 3 Pure IPC.
5. Bloque 5 UI Docentes (solo Publicaciones/Pure).
6. Bloque 4 Servicios de consolidacion.
7. Bloque 5 restante (Productos, Proyectos extendidos, Grupos).
8. Bloque 6 Seguridad y documentacion final.

## 6) Definicion de listo (DoD)

- Compila backend: cargo check OK.
- Compila frontend: npm run build OK.
- Sync Pure idempotente y sin duplicados por pure_uuid.
- Nuevos modelos PeruCRIS con indices en Mongo.
- UI funcional para Publicaciones, Productos, Equipamiento, Patentes, Financiamiento y Grupos.
- Proyecto operando en modo MongoDB como arquitectura oficial soportada.
- Reglas de cardinalidad cumplidas: proyecto permite 0..N patentes, 0..N publicaciones y 0..N productos.
- Sincronizacion Pure usa Scopus ID existente del docente como fuente oficial.

## 7) Auditoria de deuda tecnica y oportunidades

### Hallazgos criticos

1. Riesgo de duplicidad semantica de Scopus ID
- Deuda: el plan proponia agregar scopus_id en docente mientras ya existe renacyt_scopus_author_id.
- Riesgo: inconsistencias de sincronizacion y conflictos de fuente de verdad.
- Accion: mantener un solo campo canonico y, si se requiere alias, resolverlo solo en capa DTO/servicio.

2. Riesgo de acoplar comando IPC a datos de UI
- Deuda: firma inicial sync_publicaciones_pure(docente_id, scopus_id, state) fuerza paso manual de identificador.
- Riesgo: errores humanos y divergencia respecto al registro del docente.
- Accion: resolver scopus id en backend desde docente_id.

3. Falta de validacion de prerequisitos antes de llamada a Pure
- Deuda: no estaba explicito validar docente existente + scopus id + PURE_API_KEY.
- Riesgo: fallos tardios y baja observabilidad.
- Accion: validacion temprana con errores tipados y mensajes accionables.

### Hallazgos medios

4. Falta de politica de re-sync incremental
- Deuda: se define upsert idempotente pero no ventana incremental por ultima_actualizacion.
- Oportunidad: reducir costo de API y tiempo de sincronizacion.

5. Falta de estrategia de deduplicacion secundaria
- Deuda: solo pure_uuid unique.
- Oportunidad: usar claves de soporte (doi normalizado, handle) para analitica de colisiones.

6. Falta de criterios de permisos por operacion Pure
- Deuda: no define rol minimo para sync.
- Oportunidad: alinear con el patron actual de permisos en storage (docentes.manage o equivalente nuevo).

### Hallazgos bajos

7. Observabilidad insuficiente para soporte
- Deuda: faltan metricas funcionales de sync (total recibidas, insertadas, actualizadas, fallidas).
- Oportunidad: exponer resumen por ejecucion para UI y diagnostico.

8. Contratos TS/IPC no versionados
- Deuda: el plan no define version de payloads para evolucion futura.
- Oportunidad: incluir campos opcionales de compatibilidad para evitar breaking changes tempranos.

## 8) Auditoria tecnica (ronda 2) - backlog priorizado

### P0 (bloqueantes antes de despliegue Pure)

1. Eliminar panico potencial en sincronizacion
- Evidencia tecnica: uso de unwrap en calculo de timestamp durante log de conflictos.
- Riesgo: caida del proceso en escenarios de reloj/sistema anomalo.
- Accion: reemplazar unwrap por manejo seguro con map_err o fallback controlado.

2. Definir y aplicar permiso minimo para sync de publicaciones
- Evidencia tecnica: el plan define comando, pero no matriz de autorizacion final para operacion Pure.
- Riesgo: comando expuesto sin control consistente con el modelo de permisos existente.
- Accion: exigir permiso docentes.manage (o uno nuevo publicaciones.sync) en storage/service.

3. Endurecer prerequisitos de sync por docente
- Evidencia tecnica: la regla de Scopus canonico ya esta definida, pero falta checklist transaccional explicito.
- Riesgo: errores intermitentes por datos incompletos.
- Accion: validar en orden: docente existe -> docente activo -> scopus disponible -> PURE_API_KEY disponible -> llamada API.

### P1 (alta prioridad, reduce deuda estructural)

4. Reducir deuda de codigo inactivo en setup/config
- Evidencia tecnica: multiples allow(dead_code) en setup_wizard y validadores no integrados.
- Riesgo: deriva funcional y costo de mantenimiento.
- Accion: integrar wizard en flujo real o moverlo a modulo feature-gated con fecha de retiro.

5. Alinear mensajes de seguridad con estrategia Mongo-only
- Evidencia tecnica: comandos de seguridad y guia aun promueven SQLite como opcion operativa principal.
- Riesgo: contradiccion con Fase 0 de transicion a Mongo-only.
- Accion: actualizar mensajes para posicionar SQLite solo como legacy/offline temporal.

6. Formalizar contrato de salida del sync
- Evidencia tecnica: no hay contrato pactado de resumen de sincronizacion para UI.
- Riesgo: baja observabilidad, soporte reactivo.
- Accion: respuesta estructurada con conteos: fetched, inserted, updated, skipped, failed.

### P2 (mejoras recomendadas)

7. Estrategia de sincronizacion incremental
- Oportunidad: filtrar por ultima_actualizacion cuando endpoint lo permita.
- Beneficio: menor consumo de API y menor latencia.

8. Estrategia de resiliencia HTTP
- Oportunidad: retries con backoff para 429/5xx y timeout explicito.
- Beneficio: menos fallos transitorios en sincronizacion.

9. Capa de pruebas automatizadas minima
- Oportunidad: pruebas de parseo Pure y pruebas de servicio para reglas de Scopus.
- Beneficio: reduce regresiones al evolucionar contratos de API.

### Indicadores de cierre recomendados

- P0: 100% cerrado antes de habilitar boton "Sincronizar Pure" en produccion.
- P1: >= 80% cerrado antes de liberar modulo completo PeruCRIS.
- P2: planificado por sprint con minimo 1 mejora de resiliencia + 1 mejora de pruebas por iteracion.

## 9) Auditoria tecnica (ronda 3) - precision funcional solicitada

### 9.1 Aclaracion: que significa "permisos de sincronizacion"

En este plan, "permisos de sincronizacion" significa **autorizacion funcional dentro de PJUPI** (roles y permisos de negocio), no permisos del sistema operativo.

- Capa de control esperada: storage/service (patron actual de require_permission).
- Recomendacion para Pure:
   - permiso minimo: docentes.manage
   - alternativa escalable: nuevo permiso publicaciones.sync

Nota tecnica:

- Esto es independiente de Tauri capabilities; las capabilities habilitan acceso tecnico del runtime, mientras que los permisos de negocio deciden que usuario puede ejecutar el comando.

### 9.2 Integracion correcta con Pure API (validado con OpenAPI de la instancia)

Evidencia tecnica observada en la especificacion de la instancia:

- OpenAPI 3.0.1
- server base: /ws/api
- security scheme global: api-key en header (name: api-key)
- endpoint listado: GET /research-outputs (size, offset, order)
- endpoint de consulta: POST /research-outputs/search con body ResearchOutputsQuery
- endpoint de personas: POST /persons/search con body PersonsQuery

Implicacion para implementacion:

1. No usar endpoint directo sin api-key (retorna 401).
2. Implementar cliente sobre endpoint de query (POST /research-outputs/search), no solo listado basico.
3. Soportar paginacion por size/offset y corte por lotes.

### 9.3 Flujo recomendado (similar al patron RENACYT)

Objetivo: que publicaciones se obtengan y persistan de forma parecida a RENACYT/formacion academica: consulta externa -> mapeo -> persistencia interna -> metadata de sincronizacion.

Flujo:

1. UI invoca sync_publicaciones_pure(docente_id).
2. Backend valida permiso funcional.
3. Backend carga docente por docente_id y toma renacyt_scopus_author_id.
4. Si falta Scopus ID: error accionable, sin llamada a Pure.
5. Resolver investigador en Pure (preferente): POST /persons/search con searchString basado en Scopus ID.
6. Consultar publicaciones: POST /research-outputs/search con searchString y paginacion.
7. Mapear cada item (uuid, title, type, publicationStatuses, electronicVersions, modifiedDate, identifiers).
8. Upsert en Mongo por pure_uuid (idempotente).
9. Guardar metadata de sync en docente/publicacion (last_sync_at, last_sync_status, total_sync).
10. Retornar resumen estructurado para UI (fetched/inserted/updated/skipped/failed).

### 9.4 Deuda tecnica detectada en enfoque Pure y oportunidades

P0:

1. Contrato de comando aun ambiguo en permisos y salida
- Accion: cerrar DTO de entrada/salida de sync con metricas y errores parciales.

2. Ausencia de politica de paginacion obligatoria
- Accion: definir size fijo y loop por offset hasta agotar resultados.

3. Falta estrategia de vinculacion Publicacion <-> Proyecto
- Accion: mantener proyecto_id opcional inicialmente; agregar proceso de vinculacion posterior por coincidencia de proyectos asociados en payload Pure.

P1:

4. Falta de estado de sincronizacion por docente
- Accion: registrar ultima sincronizacion, ultima falla y contador de publicaciones sincronizadas.

5. Falta de retries para 429/5xx
- Accion: backoff exponencial con max intentos y timeout por request.

P2:

6. Falta de pruebas de mapeo con payloads reales
- Accion: fixtures JSON de Pure para tests de parser y upsert.

### 9.5 MongoDB principal + SQLite uso minimo (ajuste final)

Directriz operativa final:

- MongoDB = base de datos principal y fuente de verdad.
- SQLite = uso minimo y acotado a:
   - contingencia offline temporal
   - outbox/telemetria de sincronizacion cuando aplique
   - nunca como backend primario funcional para nuevas features

Acciones concretas de arquitectura:

1. Deshabilitar nuevas rutas de negocio que dependan de sqlite primary.
2. Limitar SQLite a tablas de soporte offline/sync (sin ampliar CRUD canonico).
3. Priorizar toda nueva entidad PeruCRIS/Pure directamente en Mongo.
4. Mantener ventana de transicion controlada y retirar ramas is_sqlite_primary() en servicios objetivo.

## 10) Estado actual de implementación (sesión 23 abril 2026)

### Compilación y estado general

- ✅ Backend: cargo check sin errores
- ✅ Frontend: tsc --noEmit sin errores
- ✅ Backend compila limpiamente (solo dead_code warnings para futures domain structs - esperado)
- ✅ Frontend compila limpiamente

### P0 - Deuda técnica resuelta

- ✅ **sync_service.rs:281** - Reemplazado `unwrap()` con `unwrap_or_default()` en cálculo de timestamp
- ✅ **pure_cmd.rs** - Implementado con validación temprana de permisos (docentes.manage) y scopus_id 
- ✅ **Permisos** - Aplicados correctamente en pure_cmd mediante `storage::require_docentes_manage_permission()`
- ✅ **PURE_API_KEY** - Integrado en config.rs como PureConfig, retorna error controlado si falta

### Bloque 1 ✅ COMPLETADO

Dominio Rust:
- ✅ Creados 6 domain files: publicacion.rs, patente.rs, producto.rs, equipamiento.rs, financiamiento.rs, grupo_investigacion.rs
- ✅ Extendido domain/mod.rs con imports de nuevos módulos
- ✅ Extendido Docente: campo `grupo_investigacion_id: Option<String>` con `#[sqlx(skip)]`
- ✅ Extendido Proyecto: campos `campo_ocde` y `programas_relacionados` con `#[sqlx(skip)]`
- ✅ Todos los constructores `.new()` actualizados con nuevos campos
- ✅ DTOs cuentan con SyncPublicacionesResult para conteo de metricas
- ✅ Todas las entidades usan i64 epoch ms para fechas (consistente)

### Bloque 2 ✅ COMPLETADO

MongoDB infraestructura:
- ✅ Índices creados en mongo_repo.rs para:
  - publicaciones (unique: pure_uuid, indices: docente_id, proyecto_id)
  - patentes (indices: proyecto_id, docente_id, numero_patente)
  - productos (indices: proyecto_id, docente_id)
  - equipamientos (indice: proyecto_id)
  - financiamientos (indice: proyecto_id)
  - grupos_investigacion (unique: id_grupo, indice: coordinador_id)
- ✅ ensure_indexes() ejecuta sin fallos en startup

### Bloque 3 ✅ COMPLETADO

Pure API + IPC:
- ✅ infrastructure/pure_client.rs:
  - Cliente HTTP con headers api-key + Accept
  - Método `resolve_person_uuid()` - POST /persons/search
  - Método `fetch_research_outputs_by_scopus_id()` - POST /research-outputs/search con paginación (page_size=50)
  - Mapeo defensivo de ResearchOutput → FetchedPublication
  - DTOs con `#[derive(Default)]` para Deserialize sin panicos
- ✅ commands/pure_cmd.rs:
  - `sincronizar_publicaciones_pure(docente_id)` - validación → API → upsert por pure_uuid
  - `get_publicaciones_docente(docente_id)` - lectura de colección
  - Upsert idempotente: $set + $setOnInsert, con resultado .upserted_id tracking (nuevas vs actualizadas)
  - Respuesta SyncPublicacionesResult con conteos estructurados
- ✅ Config:
  - PureConfig registrado en config.rs y state.rs
  - RuntimeConfig::pure field incluido
  - PJUPI_PURE_API_BASE_URL y PJUPI_PURE_API_KEY en merge_process_env
- ✅ IPC wiring:
  - pure_cmd registrado en commands/mod.rs
  - Comandos wired en lib.rs invoke_handler

### Bloque 4 (servicios) ⏳ PARCIAL

- ⏳ Requisito: "servicios con lógica", no comandos delgados
- Nota: Actualmente pure_cmd invoca directamente db.collection() en lugar de un service
- Recomendación: Crear services/publicaciones_service.rs para aislar lógica de negocio
- Status: Funcional pero refactor recomendado para P1

### Bloque 5 ✅ COMPLETADO (Docentes + tipos TS)

Frontend TypeScript + React:
- ✅ types.ts:
  - Interfaces añadidas: Publicacion, SyncPublicacionesResult, Patente, Producto, Equipamiento, Financiamiento, GrupoInvestigacion
- ✅ services/tauri/pure.ts:
  - Wrapper IPC: sincronizarPublicacionesPure(), getPublicacionesDocente()
- ✅ features/docentes/api.ts:
  - Re-exporta nuevos IPC + tipos de Pure
- ✅ DocenteDetailModal.tsx:
  - Nueva sección "Publicaciones (Pure)" con toggle expandible
  - Validación: aviso si docente carece de scopus_id (no intenta sync)
  - Botón "Sincronizar desde Pure" (condicionado a canSyncPure + tieneScopusId)
  - Carga lazy de publicaciones al expandir
  - Listado con título, año, tipo, journal, DOI (link externo), autores (parsed JSON)
  - Manejo de errores con toast
  - Estado de carga (isSyncingPure) con UI de botón deshabilitado
- ✅ DocentesTable.tsx:
  - Pasa canSyncPure={canManage} al modal
- ✅ App.css:
  - Clase utility `renacyt-formacion-full-col` (grid-column: 1 / -1) para autores a ancho completo

### Bloque 5 ⏳ PENDIENTE (Proyectos + Grupos)

Proyectos:
- ⏳ ProyectoCreateModal/EditModal: agregar secciones opcionales para Patentes, Productos, Equipamiento, Financiamiento
- ⏳ Validación: permitir 0 elementos por sección (todas opcionales)

Grupos de Investigación:
- ⏳ Crear src/features/grupos/GruposTab.tsx
- ⏳ Integrar tab en App.tsx (permisos y lazy load)
- ⏳ API: operaciones CRUD en features/grupos/api.ts

### Bloque 6 ⏳ PENDIENTE (Seguridad y documentación)

Seguridad:
- ⏳ Auditar logs de pure_cmd/pure_client para evitar exposición de PURE_API_KEY
- ⏳ Considerar config_validator.rs para validación de PURE_API_KEY en startup (actual: falla gracefully cuando se invoca comando)

Documentación:
- ⏳ Actualizar README con variables PJUPI_PURE_*
- ⏳ Documentar default.env con ejemplos y comentarios
- ⏳ Guía de configuración Pure API para usuarios

### Resumen de DTO finales implementados

```typescript
// Backend (Rust DTOs)
SyncPublicacionesResult {
  docente_id: String,
  scopus_author_id: String,
  pure_person_uuid: Option<String>,
  total_encontradas: usize,
  nuevas: usize,
  actualizadas: usize,
}

Publicacion {
  id_publicacion: String (UUID),
  pure_uuid: String (unique PK from Pure),
  docente_id: String (canonical source),
  proyecto_id: Option<String>,
  titulo: String,
  tipo_publicacion: Option<String>,
  doi: Option<String>,
  scopus_eid: Option<String>,
  anio_publicacion: Option<i32>,
  autores_json: Option<String> (JSON array),
  estado_publicacion: Option<String>,
  journal_titulo: Option<String>,
  issn: Option<String>,
  pure_sincronizado_at: Option<i64> (ms epoch),
  created_at: Option<i64>,
  updated_at: Option<i64>,
}

// Frontend (TS equivalents in types.ts)
// Nombres idénticos, tipos opcionales con | null

// IPC Signatures (client.ts)
sincronizar_publicaciones_pure(docente_id: string) → SyncPublicacionesResult
get_publicaciones_docente(docente_id: string) → Publicacion[]
```

### Checklist de próximas sesiones

#### Bloque 5 restante (Proyectos + Grupos)

- [ ] Crear GruposTab.tsx con tabla CRUD para grupos de investigación
- [ ] Extender App.tsx para agregar tab Grupos con permisos (admin/operador)
- [ ] Crear API wrapper en features/grupos/api.ts
- [ ] Extender Proyecto modal con secciones opcionales (form-array pattern)
- [ ] Backend: crear commands para CRUD de grupos (crear_grupo, get_all_grupos, etc.)

#### Bloque 6 (Seguridad)

- [ ] Auditar pure_client.rs/pure_cmd.rs para logs seguros (sin key expuesta)
- [ ] Validar que error messages no filtren PURE_API_KEY
- [ ] Actualizar setup_wizard.rs para incluir campo PURE_API_KEY (optativo)
- [ ] Crear default.env con placeholders y comentarios
- [ ] Actualizar README.md con sección "Configuración Pure"

#### P1 - Refactoring recomendado

- [ ] Crear services/publicaciones_service.rs para aislar lógica de upsert
- [ ] Reutilizar en pure_cmd (llamar a service en lugar de direct mongo)
- [ ] Aplicar patrón a futuras operaciones de Patentes/Productos/etc.

#### P2 - Resilencia + pruebas

- [ ] Agregar retry logic con backoff a pure_client.rs (429, 5xx)
- [ ] Timeout explícito en reqwest calls (actualmente: por defecto)
- [ ] Pruebas unitarias minimas: parseResearchOutput con fixtures
- [ ] Integración end-to-end: auth → sync → verificar Mongo (mock Pure)

### Métricas de completitud

| Bloque | Status | Completitud | Bloqueantes para MVP |
|--------|--------|------------|----------------------|
| 1 - Dominio | ✅ | 100% | Ninguno |
| 2 - Infra MongoDB | ✅ | 100% | Ninguno |
| 3 - Pure IPC | ✅ | 100% | Ninguno |
| 4 - Servicios | ⏳ | 50% | Refactor recomendado (no bloqueante) |
| 5a - Frontend Docentes | ✅ | 100% | Ninguno |
| 5b - Frontend Proyectos | ⏳ | 0% | Bloquea despliegue completo |
| 5c - Frontend Grupos | ⏳ | 0% | Bloquea despliegue completo |
| 6 - Seguridad | ⏳ | 20% | Auditoría crítica para prod |

**MVP actual funcional**: Sincronización Pure de publicaciones por docente, visualización en UI de Docentes (✅ listo para demostración interna)

**Ruta a MVP completo**: Completar Proyectos (5b) + Grupos (5c) + seguridad (6) en ~1-2 sesiones adicionales.

