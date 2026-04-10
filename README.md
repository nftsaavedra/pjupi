# PJUPI

Aplicación de escritorio construida con Tauri, React y TypeScript para gestionar grados académicos, docentes, proyectos y reportes.

## Backend de Base de Datos

La aplicación ahora soporta dos backends:

- `sqlite`: modo heredado y fallback local.
- `mongodb`: nuevo backend principal para migración gradual.

La selección se hace por variables de entorno en `.env`:

```env
PJUPI_DB_BACKEND=mongodb
PJUPI_MONGODB_URI=<tu_uri_mongodb>
PJUPI_MONGODB_DB=pjupi
PJUPI_SQLITE_URL=sqlite:database.db
PJUPI_RENIEC_API_BASE_URL=https://api.decolecta.com/v1
PJUPI_RENIEC_TOKEN=<tu_token_reniec>
```

## Integración RENIEC

- La consulta de DNI usa por defecto `https://api.decolecta.com/v1`.
- Solo necesita definir `PJUPI_RENIEC_TOKEN` para habilitar el autocompletado de datos personales en el alta de docentes.
- Si el token no está configurado, el flujo manual sigue funcionando y la aplicación mostrará un mensaje claro al intentar consultar.

El formulario de docentes mantiene compatibilidad con el flujo actual, pero ahora registra nombres y apellidos por separado y conserva `nombres_apellidos` como valor compuesto para listados, reportes y trazabilidad existente.

## Estrategia de Migración Segura

Al iniciar en modo `mongodb`, el backend hace lo siguiente:

1. Conecta a MongoDB y asegura índices únicos.
2. Conserva SQLite como fuente de respaldo si existe `database.db`.
3. Ejecuta una migración `one-shot` desde SQLite hacia MongoDB solo si Mongo está vacío y la migración no fue registrada antes.
4. No borra ni altera el archivo SQLite original, por lo que el rollback sigue siendo posible.

La metadata de migración se guarda en la colección `system_meta`.

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
- En Windows, Tauri requiere Microsoft Edge WebView2 Runtime. Si la PC destino no lo tiene, la app puede no abrir hasta instalarlo.
- El backend SQLite local ahora se guarda por defecto en la carpeta de usuario (`%LOCALAPPDATA%\pjupi\database.db` en Windows), no dentro del directorio de instalación.

## Verificación

```bash
cd src-tauri && cargo check
npm run build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
