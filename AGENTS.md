# PJUPI — Agentes de Desarrollo

## Identidad del Proyecto

Sistema de escritorio para gestión de investigación universitaria (docentes, proyectos, grados
académicos, grupos de investigación, reportes). Construido con Tauri v2 + React + Rust.

- **Nombre**: PJUPI (UPIC/UNF)
- **Tipo**: Desktop app (Windows)
- **Versión**: 0.1.0 — etapa de construcción/consolidación
- **Identificador**: `com.upic.pjupi`

---

## Stack Tecnológico

| Capa | Tecnología | Versión |
|------|-----------|---------|
| Shell | Tauri v2 | 2.10.x |
| Frontend | React + TypeScript | 19.1 / 6.0 |
| Bundler | Vite (Rolldown) | 8.0 |
| Backend | Rust (edition 2021) | 1.85+ |
| Base de datos | MongoDB Atlas | Driver 3.5 |
| Diseño | DESIGN.md (Google format) | alpha |
| Auth | Argon2 (password hashing) | 0.5.3 |
| HTTP | reqwest (rustls-tls) | 0.12 |
| Gráficos | recharts | 3.8 |
| PDF | @react-pdf/renderer | 4.4 |
| Excel | exceljs | 4.4 |
| Iconos | lucide-react | 1.7 |
| Tooltips | @floating-ui/react | 0.27 |

---

## Estructura del Proyecto (Screaming Architecture)

```
pjupi/
├── src/                          # Frontend React
│   ├── features/                 # Módulos de dominio (vertical slicing)
│   │   ├── auth/                 #   Login / primer arranque
│   │   ├── dashboard/            #   KPIs y gráficos
│   │   ├── docentes/             #   CRUD docentes + RENIEC/RENACYT/Pure
│   │   ├── proyectos/            #   CRUD proyectos + participantes + recursos
│   │   ├── grupos/               #   Grupos de investigación
│   │   ├── reportes/             #   Exportación Excel/PDF
│   │   └── configuracion/        #   Grados académicos + Usuarios
│   ├── shared/                   # Componentes transversales
│   │   ├── auth/                 #   RBAC (roles, permisos)
│   │   ├── ui/                   #   AppIcon, Skeleton, TableActionButton
│   │   ├── forms/                #   FormModal, FormInput, FormSelect, FieldHelpTooltip
│   │   ├── overlays/             #   ConfirmDialog, FloatingTooltip
│   │   ├── navigation/           #   TabNavigation
│   │   ├── feedback/             #   ToastContainer
│   │   ├── hooks/                #   useStableFetch, useRefreshToast
│   │   └── utils/                #   renacyt, saveDesktopFile
│   ├── services/                 # Capa de API (Tauri IPC wrappers)
│   │   └── tauri/                #   client, error, types, auth, docentes, proyectos, ...
│   └── hooks/                    # Barrel re-exports → features/*/hooks + shared/hooks
│
├── src-tauri/                    # Backend Rust
│   └── src/
│       ├── shared/               # Infraestructura transversal
│       │   ├── error.rs          #   AppError, sanitize_external_detail
│       │   ├── state.rs          #   AppState, SessionStore
│       │   ├── config.rs         #   Carga de configuración multi-fuente
│       │   ├── db.rs             #   Conexión MongoDB
│       │   ├── access_control.rs #   Handlers de dominio (delega RBAC a rbac.rs)
│       │   ├── rbac.rs           #   RBAC: roles, permisos, autorización
│       │   ├── audit.rs          #   Auditoría de operaciones
│       │   ├── time.rs           #   Helper de timestamps unificado
│       │   ├── data_loader.rs    #   Helpers compartidos de carga (load_*_map)
│       │   ├── logging.rs        #   Structured logging via tracing
│       │   └── external/         #   Clientes HTTP a servicios externos
│       ├── catalogos/            # Feature: Catálogos parametrizables (tipos, estados, monedas)
│       ├── docentes/             # Feature: Docentes
│       ├── proyectos/            # Feature: Proyectos + Participantes
│       ├── recursos/             # Feature: Patentes/Productos/Equipamientos/Financiamientos (repo via macros)
│       ├── reportes/             # Feature: Estadísticas + Exportación
│       └── seguridad/            # Feature: Status de seguridad + guías
│
├── docs/                         # Documentación de arquitectura
├── _docs/                        # Vault Obsidian (docs internas)
├── scripts/                      # Scripts de build (PowerShell)
└── .agents/skills/               # Skills para agentes AI
```

---

## Principios de Arquitectura

### Design System

El diseño del frontend sigue el estándar [DESIGN.md](https://github.com/google-labs-code/design.md). El archivo `DESIGN.md` define tokens de diseño (colores, tipografía, espaciado, sombras, bordes) y guías de componentes. Todo cambio visual debe usar las CSS variables de `App.css` que corresponden a los tokens de DESIGN.md.

### Screaming Architecture
La estructura de directorios **grita** lo que la aplicación HACE, no qué frameworks usa.
Cada feature es un módulo autocontenido con sus propios modelos, comandos, servicios
y repositorio.

### Single Responsibility (SRP)
- **Un archivo = Una responsabilidad**
- `mongo_repo.rs` monolítico (1320 líneas) → dividido por feature
- Hooks de React → máximo 200 líneas, una sola entidad de negocio

### DRY (Don't Repeat Yourself)
- Helpers duplicados en exportaciones (Excel/PDF) → compartidos en `shared/utils/`
- Handlers de recursos casi idénticos → abstraídos en hook genérico

### KISS (Keep It Simple)
- Sin over-engineering: MongoDB como única BD es aceptable para v0.1.0
- Sin ORMs: queries directas al driver de MongoDB
- Sin state management libraries: `useState` + custom hooks

---

## Comandos Disponibles

```bash
# Desarrollo
npm run dev              # Vite dev server (puerto 1420)
npm run tauri:dev        # Tauri dev mode (ventana desktop)

# Calidad
npm run lint             # ESLint
npm run lint:fix         # ESLint --fix
npm run format           # Prettier --write
npm run typecheck        # tsc --noEmit
npm run check            # lint + typecheck

# Build
npm run build            # tsc + vite build
npm run tauri:build:exe  # Release .exe sin bundle
npm run tauri:build:installer  # NSIS installer completo

# Rust
cargo check              # Verificar compilación (desde src-tauri/)
cargo test               # Tests unitarios (desde src-tauri/)
cargo clippy             # Linter Rust
```

---

## Convenciones de Código

### TypeScript / React
- **Imports**: Usar alias `@/` → `./src/` (ej: `@/shared/ui/AppIcon`)
- **Componentes**: PascalCase, una exportación por archivo
- **Hooks**: `use` prefix, extraer lógica de negocio del JSX
- **Tipos**: Interfaces en `services/tauri/types.ts`, tipos locales en el feature
- **Manejo de errores**: Siempre `try/catch` con `getTauriErrorMessage(error)`
- **Lazy loading**: Features grandes con `React.lazy` + `Suspense` + skeleton fallback

### Rust
- **Módulos**: Un `mod.rs` por feature, exports públicos explícitos
- **Errores**: Siempre `Result<T, AppError>`, nunca `unwrap()` en producción
- **Nombres**: snake_case para funciones, CamelCase para tipos
- **Formato**: `rustfmt.toml` (100 chars, edition 2021, group_imports)
- **Dependencias**: Mínimas, evitar crates innecesarios
- **Timestamps**: Usar `shared::time::now_ms()` (unificado, basado en `std::time`)

---

## Roles y Permisos (RBAC)

| Rol | Permisos |
|-----|----------|
| **admin** | Todo (incluye gestión de usuarios y grados) |
| **operador** | CRUD docentes, proyectos, grupos, recursos + reportes export |
| **consulta** | Solo lectura: dashboard, docentes, proyectos, reportes, grupos |

---

## Servicios Externos

| Servicio | Propósito | Configuración |
|----------|-----------|---------------|
| MongoDB Atlas | Base de datos principal | `PJUPI_MONGODB_URI` + `PJUPI_MONGODB_DB` |
| RENIEC | Consulta de DNI | `PJUPI_RENIEC_TOKEN` |
| RENACYT | Registro de investigadores | URL base + versión de acto |
| Pure (Elsevier) | Sincronización de publicaciones | `PJUPI_PURE_API_BASE_URL` + `PJUPI_PURE_API_KEY` |

---

## Flujo de Trabajo con OpenCode

### Agentes Recomendados
- **explore**: Para búsquedas en el codebase, encontrar patrones, analizar dependencias
- **general**: Para tareas multi-step complejas que requieren leer + escribir
- Usar agentes en paralelo cuando las tareas son independientes

### Skills Disponibles
- `tauri-v2`: Tauri v2 patterns, IPC, capacidades, build troubleshooting
- `vite`: Configuración de Vite, plugins, chunk splitting
- `frontend-design`: Diseño de UI/UX
- `typescript-advanced-types`: Tipos avanzados de TypeScript
- `accessibility`: Auditoría WCAG 2.2
- `vercel-react-best-practices`: Optimización de rendimiento React
- `vercel-composition-patterns`: Patrones de composición React

### Testing (Pendiente)
- Rust: `cargo test` (6 tests: 2 config_validator + 4 docentes)
- Frontend: Vitest + Testing Library (15 tests: permissions + error handling)
- E2E: Sin tests (Playwright recomendado con Tauri)

### CI/CD (Pendiente)
- GitHub Actions configurado (`.github/workflows/ci.yml`): lint + typecheck + test + build
- Build scripts en `scripts/tauri-build.ps1`

---

## Deuda Técnica Conocida

| Prioridad | Ítem |
|-----------|------|
| ✅ Resuelto | CSP habilitado en tauri.conf.json |
| ✅ Resuelto | Tests: 6 Rust + 15 frontend (Vitest) |
| ✅ Resuelto | Structured logging con tracing crate |
| ✅ Resuelto | Sin transacciones MongoDB para operaciones multi-documento |
| ✅ Resuelto | Sin paginación en queries de lista (tipo PaginatedResult creado, integrado en docentes) |
| ✅ Resuelto | `pure_cmd.rs` bypassea capa de servicios → pure_service.rs creado |
| ✅ Resuelto | `chrono` centralizado en `time.rs` y `renacyt_client.rs` |
| ✅ Resuelto | `access_control.rs` dividido en `rbac.rs` + handlers + auditoría genérica en 11 operaciones |
| 🟡 Bajo | Auditoría pendiente en recursos (12 operaciones) y update/reactivate de docentes/grados/proyectos |
| 🟡 Medio | Dropdowns de recursos aún usan placeholders; integrar con catálogos (FormSelect dinámico) |
