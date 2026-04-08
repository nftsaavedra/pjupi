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
```

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

## Verificación

```bash
cd src-tauri && cargo check
npm run build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
