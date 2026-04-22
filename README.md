# PJUPI

Aplicación de escritorio para gestión de grados académicos, docentes y proyectos de investigación. Construida con Tauri v2, React y TypeScript, con backend en Rust.

## Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust (Tauri v2)
- **Base de datos**: MongoDB (primario) + SQLite (store local / offline)

## Configuración

En desarrollo use `.env` en la raíz. En producción, edite `%APPDATA%\com.upic.pjupi\pjupi.env`.

```env
PJUPI_DB_BACKEND=mongodb
PJUPI_MONGODB_URI=mongodb://localhost:27017
PJUPI_MONGODB_DB=pjupi
PJUPI_RENIEC_TOKEN=<token_opcional>
```

| Variable | Descripción | Requerida |
|----------|-------------|-----------|
| `PJUPI_DB_BACKEND` | `mongodb` o `sqlite` | No (default: `mongodb` si hay URI) |
| `PJUPI_MONGODB_URI` | URI de conexión a MongoDB | Sí (modo MongoDB) |
| `PJUPI_MONGODB_DB` | Nombre de la base de datos | No (default: `pjupi`) |
| `PJUPI_SQLITE_URL` | Ruta personalizada para SQLite | No |
| `PJUPI_RENIEC_TOKEN` | Token API para consulta de DNI | No |

## Desarrollo

```bash
npm install
npm run tauri:dev
```

## Build

```bash
# Solo ejecutable (.exe)
npm run tauri:build:exe

# Instalador NSIS (recomendado)
npm run tauri:build:installer

# Portable (ZIP con launcher)
npm run tauri:build:portable

# Targets explícitos
npm run tauri:build:nsis
npm run tauri:build:msi
```

> `msi` requiere WiX Toolset. `nsis` es el bundle por defecto del proyecto.

## Verificación

```bash
cd src-tauri && cargo check
npm run build
```

## Arquitectura

```
Frontend (React) → Tauri IPC → Services → BackendStrategy → MongoDB / SQLite
                                                                    ↕
                                                          sync_outbox (offline queue)
```

- MongoDB es la fuente de verdad. SQLite actúa como store local para operación offline.
- Las mutaciones en modo offline se encolan en `sync_outbox` y se sincronizan al reconectar.
- Política de conflictos: MongoDB-primary.
- Ver [docs/mongodb-primary-plan.md](docs/mongodb-primary-plan.md) para detalles de arquitectura.

## IDE Recomendado

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
