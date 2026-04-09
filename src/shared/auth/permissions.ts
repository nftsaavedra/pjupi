export type AppRole = 'admin' | 'operador' | 'consulta';

export type AppPermission =
  | 'dashboard.view'
  | 'docentes.view'
  | 'docentes.manage'
  | 'proyectos.view'
  | 'proyectos.manage'
  | 'reportes.view'
  | 'reportes.export'
  | 'configuracion.view'
  | 'grados.manage'
  | 'usuarios.manage';

interface RoleDefinition {
  label: string;
  summary: string;
  permissions: AppPermission[];
  capabilities: string[];
}

export const ROLE_DEFINITIONS: Record<AppRole, RoleDefinition> = {
  admin: {
    label: 'Administrador',
    summary: 'Control total del sistema, accesos y catálogos base.',
    permissions: [
      'dashboard.view',
      'docentes.view',
      'docentes.manage',
      'proyectos.view',
      'proyectos.manage',
      'reportes.view',
      'reportes.export',
      'configuracion.view',
      'grados.manage',
      'usuarios.manage',
    ],
    capabilities: [
      'Gestiona usuarios, roles y estado de acceso.',
      'Administra grados académicos y todo el dato operativo.',
      'Puede crear, actualizar, desactivar, reactivar y exportar.',
    ],
  },
  operador: {
    label: 'Operador',
    summary: 'Gestión operativa diaria de docentes y proyectos.',
    permissions: [
      'dashboard.view',
      'docentes.view',
      'docentes.manage',
      'proyectos.view',
      'proyectos.manage',
      'reportes.view',
      'reportes.export',
    ],
    capabilities: [
      'Gestiona docentes, proyectos y sincronizaciones operativas.',
      'Consulta dashboard y reportes con opción de exportar.',
      'No administra usuarios ni catálogos de configuración.',
    ],
  },
  consulta: {
    label: 'Consulta',
    summary: 'Acceso de solo lectura a la información operativa.',
    permissions: [
      'dashboard.view',
      'docentes.view',
      'proyectos.view',
      'reportes.view',
    ],
    capabilities: [
      'Visualiza dashboard, docentes, proyectos y reportes.',
      'No puede crear, editar, desactivar, reactivar ni sincronizar.',
      'No puede exportar ni acceder a configuración.',
    ],
  },
};

export const normalizeAppRole = (value: string | null | undefined): AppRole => {
  const normalizedValue = (value ?? '').trim().toLowerCase();

  if (normalizedValue === 'admin' || normalizedValue === 'operador' || normalizedValue === 'consulta') {
    return normalizedValue;
  }

  return 'consulta';
};

export const getRoleLabel = (value: string | null | undefined) => ROLE_DEFINITIONS[normalizeAppRole(value)].label;

export const getRoleDefinition = (value: string | null | undefined) => ROLE_DEFINITIONS[normalizeAppRole(value)];

export const hasPermission = (role: string | null | undefined, permission: AppPermission) => (
  ROLE_DEFINITIONS[normalizeAppRole(role)].permissions.includes(permission)
);

export const getRoleOptions = () => (
  (Object.entries(ROLE_DEFINITIONS) as Array<[AppRole, RoleDefinition]>).map(([value, definition]) => ({
    value,
    label: definition.label,
  }))
);