# PJUPI

Aplicación de escritorio construida con Tauri, React y TypeScript para gestionar grados académicos, docentes, proyectos y reportes.

## Backend de Base de Datos

La aplicación ahora soporta dos backends:

- `mongodb`: backend principal y fuente de verdad objetivo.
- `sqlite`: respaldo local, soporte offline y compatibilidad operativa.

La configuración actual se define en `pjupi.env` en ejecución instalada o portable, y en `.env` durante desarrollo. Valores soportados:

```env
PJUPI_DB_BACKEND=mongodb
PJUPI_MONGODB_URI=<tu_uri_mongodb>
PJUPI_MONGODB_DB=pjupi
PJUPI_SQLITE_URL=sqlite:database.db
PJUPI_RENIEC_API_BASE_URL=https://api.decolecta.com/v1
PJUPI_RENIEC_TOKEN=<tu_token_reniec>
```

Estado actual del proyecto:

- MongoDB ya está tratado como backend preferente a nivel de configuración.
- La capa de aplicación ya migró a servicios por agregado para reducir el `dispatch` repetido por backend.
- SQLite ahora se inicializa como store local aún cuando MongoDB es el backend primario.
- Las mutaciones de `grados`, `proyectos`, `docentes` y `usuarios` en modo SQLite primario ya se encolan en `sync_outbox`.
- Todas las mutaciones se registran como snapshots de estado para sincronización.
- En arranque, si hay SQLite local y MongoDB disponible, se ejecuta sincronización inicial de outbox pendiente.
- **Política de resolución de conflictos**: MongoDB-primary. En caso de conflicto, MongoDB prevalece.
- La refactorización pendiente consiste en eliminar el `dispatch` dentro de los servicios y restringir escrituras online en SQLite.

Plan documentado:

- Ver [docs/mongodb-primary-plan.md](docs/mongodb-primary-plan.md).
- Fases completadas: 1 (dual-backend congelado), 2 (servicios por agregado), 3 (SQLite especializado con outbox/sync para todos los agregados).
- Fases en progreso: 4 (simplificación de dispatch), 5 (offline real con conflictos), 6 (hardening).

## Integración RENIEC

- La consulta de DNI usa por defecto `https://api.decolecta.com/v1`.
- Solo necesita definir `PJUPI_RENIEC_TOKEN` para habilitar el autocompletado de datos personales en el alta de docentes.
- Si el token no está configurado, el flujo manual sigue funcionando y la aplicación mostrará un mensaje claro al intentar consultar.

El formulario de docentes mantiene compatibilidad con el flujo actual, pero ahora registra nombres y apellidos por separado y conserva `nombres_apellidos` como valor compuesto para listados, reportes y trazabilidad existente.

## Estrategia de Transición

La dirección objetivo del proyecto es:

1. MongoDB como backend principal.
2. SQLite como almacenamiento local para contingencia, trabajo offline o caché operativa.
3. Sin paridad obligatoria de nuevas reglas de negocio entre dos repositorios principales.

Mientras se completa la refactorización, al iniciar en modo `mongodb` el backend hace lo siguiente:

1. Conecta a MongoDB y asegura índices únicos.
2. Conserva SQLite como fuente de respaldo si existe `database.db`.
3. Ejecuta una migración `one-shot` desde SQLite hacia MongoDB solo si Mongo está vacío y la migración no fue registrada antes.
4. No borra ni altera el archivo SQLite original, por lo que el rollback sigue siendo posible.

La metadata de migración se guarda en la colección `system_meta`.

Limitación actual:

- Aunque la estrategia ya apunta a MongoDB-first, todavía existen repositorios concretos separados para SQLite y MongoDB.
- El `dispatch` ya salió de varios casos de uso en `storage`, pero sigue existiendo en la capa de servicios mientras se completa la unificación.
- SQLite todavía conserva CRUD legado completo; `sync_outbox` y `sync_state` ya están operativas para `grados`, `proyectos` y `docentes`, pero la sincronización completa del resto de agregados aún no está terminada.

## Desarrollo

```bash
npm install
npm run tauri:dev
```

## Build de Escritorio en Windows

Para evitar fallos de empaquetado cuando la red está restringida o GitHub no es resoluble:

```bash
npm run tauri:build:exe
```

Ese comando compila el ejecutable sin generar instalador.

Si necesita instalador, el flujo recomendado ahora es NSIS por defecto:

```bash
npm run tauri:build:installer
```

Si necesita una version portable sin instalador:

```bash
npm run tauri:build:portable
```

Ese comando genera:

- una carpeta portable con `pjupi.exe`
- un lanzador `Iniciar PJUPI.cmd`
- un `LEEME-PORTABLE.txt`
- un ZIP listo para copiar a otra PC

Comandos explícitos:

```bash
npm run tauri:build:nsis
npm run tauri:build:msi
```

Notas:

- `msi` requiere WiX Toolset disponible localmente o acceso de red para descargarlo si no está cacheado.
- `nsis` evita la dependencia de WiX y es el bundle por defecto del proyecto.
- Los scripts de build intentan reutilizar las herramientas cacheadas por Tauri en Windows antes de intentar cualquier descarga.
- La PC destino no necesita Rust instalado para ejecutar el instalador o el `.exe` generado.
- El instalador NSIS ahora incluye el flujo oficial de Tauri para asegurar WebView2 en Windows usando `webviewInstallMode`.
- El backend SQLite local ahora se guarda por defecto en la carpeta de usuario (`%LOCALAPPDATA%\pjupi\database.db` en Windows), no dentro del directorio de instalación.

## Configuración en Producción

En desarrollo, la app sigue leyendo `.env` del workspace si existe.

En build instalado o portable, la app ya no depende del `.env` del proyecto:

1. Tauri empaqueta `.env` como recurso interno siguiendo el mecanismo oficial `bundle.resources`.
2. En el primer arranque, la app copia esa configuración a un archivo editable por usuario.
3. Desde entonces, la app lee la configuración desde ese archivo local y las variables de entorno del sistema, si existen, tienen prioridad.

Ubicación esperada del archivo editable en Windows:

```text
%APPDATA%\com.upic.pjupi\pjupi.env
```

Esto permite un flujo más razonable para desktop:

- valores por defecto empaquetados para validación inicial
- configuración editable luego de instalar
- posibilidad de reemplazar credenciales sin recompilar

Recomendación operativa actual:

- En despliegues conectados, use `PJUPI_DB_BACKEND=mongodb`.
- Reserve `sqlite` para escenarios de contingencia, pruebas locales aisladas o futura operación offline.

Importante:

- Para una versión preliminar, esto ayuda a validar despliegue.
- Para producción real, no es recomendable distribuir credenciales sensibles de MongoDB o tokens de terceros dentro del cliente de escritorio. Lo correcto es mover esos secretos a un backend controlado o a un flujo de provisión seguro.

## Arquitectura Objetivo de Datos

Objetivo de mediano plazo:

- MongoDB concentra escritura, lectura principal, índices y reglas de negocio persistentes.
- SQLite se usa como réplica local parcial o cola de trabajo offline.
- La sincronización se hace desde una capa dedicada, no desde duplicación total de repositorios.

Síntomas actuales que justifican la refactorización:

- La capa [src-tauri/src/storage.rs](src-tauri/src/storage.rs) ya redujo el `dispatch` directo, pero la selección de backend todavía existe en la capa de servicios.
- Existen repositorios de dominio paralelos en SQLite y MongoDB con lógica de negocio repetida.
- El costo de mantener paridad funcional entre ambos backends crece con cada cambio de dominio.

## Verificación

```bash
cd src-tauri && cargo check
npm run build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
