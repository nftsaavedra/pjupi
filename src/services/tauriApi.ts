import { invoke } from '@tauri-apps/api/core';

export const getTauriErrorMessage = (error: unknown): string => {
  if (!error) return 'Error desconocido';
  if (typeof error === 'string') return error;

  if (typeof error === 'object') {
    const maybe = error as Record<string, unknown>;

    // Common Error object shape
    if (typeof maybe.message === 'string' && maybe.message.trim()) {
      return maybe.message;
    }

    // Rust AppError enum serialized by Tauri
    const keys = [
      'DatabaseError',
      'UniqueConstraintViolation',
      'NotFound',
      'InternalError',
      'ConfigurationError',
    ];
    for (const key of keys) {
      const value = maybe[key];
      if (typeof value === 'string' && value.trim()) {
        return value;
      }
    }

    // Fallback for nested structures
    try {
      return JSON.stringify(error);
    } catch {
      return String(error);
    }
  }

  return String(error);
};

// Types matching Rust DTOs
export interface Docente {
  id_docente: string;
  dni: string;
  id_grado: string;
  nombres_apellidos: string;
  activo?: number;
}

export interface Proyecto {
  id_proyecto: string;
  titulo_proyecto: string;
}

export interface ProyectoDetalle {
  id_proyecto: string;
  titulo_proyecto: string;
  cantidad_docentes: number;
  docentes: string | null;
  activo: number;
}

export interface EliminarProyectoResultado {
  accion: 'desactivado' | string;
  mensaje: string;
}

export interface GradoAcademico {
  id_grado: string;
  nombre: string;
  descripcion?: string;
  activo?: number;
}

export interface EliminarGradoResultado {
  accion: 'eliminado' | 'desactivado' | string;
  mensaje: string;
}

export interface DocenteProyectosCount {
  nombre: string;
  cantidad: number;
}

export interface KpisDashboard {
  total_proyectos: number;
  total_docentes: number;
  docentes_con_1_proyecto: number;
  docentes_multiples_proyectos: number;
}

export interface ExportData {
  proyecto: string;
  grado: string;
  docente: string;
  dni: string;
}

// Docente API
export const crearDocente = async (dni: string, id_grado: string, nombres_apellidos: string): Promise<Docente> => {
  return await invoke('crear_docente', { request: { dni, id_grado, nombres_apellidos } });
};

export const getAllDocentes = async (): Promise<Docente[]> => {
  return await invoke('get_all_docentes');
};

// NEW: Get docentes with project details
export interface DocenteDetalle {
  id_docente: string;
  dni: string;
  nombres_apellidos: string;
  grado: string;
  cantidad_proyectos: number;
  proyectos: string | null;
  activo: number;
}

export interface EliminarDocenteResultado {
  accion: 'desactivado' | string;
  mensaje: string;
}

export const getAllDocentesConProyectos = async (): Promise<DocenteDetalle[]> => {
  return await invoke('get_all_docentes_con_proyectos');
};

export const eliminarDocente = async (id_docente: string): Promise<EliminarDocenteResultado> => {
  return await invoke('eliminar_docente', { idDocente: id_docente });
};

export const reactivarDocente = async (id_docente: string): Promise<Docente> => {
  return await invoke('reactivar_docente', { idDocente: id_docente });
};

// Proyecto API
export const crearProyectoConParticipantes = async (titulo_proyecto: string, docentes_ids: string[]): Promise<Proyecto> => {
  return await invoke('crear_proyecto_con_participantes', { request: { titulo_proyecto, docentes_ids } });
};

export const buscarProyectosPorDocente = async (id_docente: string): Promise<Proyecto[]> => {
  return await invoke('buscar_proyectos_por_docente', { idDocente: id_docente });
};

export const getAllProyectosDetalle = async (): Promise<ProyectoDetalle[]> => {
  return await invoke('get_all_proyectos_detalle');
};

export const eliminarRelacionProyectoDocente = async (id_proyecto: string, id_docente: string): Promise<void> => {
  return await invoke('eliminar_relacion_proyecto_docente', { idProyecto: id_proyecto, idDocente: id_docente });
};

export const eliminarRelacionesProyecto = async (id_proyecto: string): Promise<void> => {
  return await invoke('eliminar_relaciones_proyecto', { idProyecto: id_proyecto });
};

export const eliminarProyecto = async (id_proyecto: string): Promise<EliminarProyectoResultado> => {
  return await invoke('eliminar_proyecto', { idProyecto: id_proyecto });
};

export const reactivarProyecto = async (id_proyecto: string): Promise<Proyecto> => {
  return await invoke('reactivar_proyecto', { idProyecto: id_proyecto });
};

// Grado Académico API
export const getAllGrados = async (): Promise<GradoAcademico[]> => {
  return await invoke('get_all_grados');
};

export const crearGrado = async (nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('crear_grado', { request: { nombre, descripcion } });
};

export const actualizarGrado = async (id_grado: string, nombre: string, descripcion?: string): Promise<GradoAcademico> => {
  return await invoke('actualizar_grado', { idGrado: id_grado, request: { nombre, descripcion } });
};

export const eliminarGrado = async (id_grado: string): Promise<EliminarGradoResultado> => {
  return await invoke('eliminar_grado', { idGrado: id_grado });
};

export const reactivarGrado = async (id_grado: string): Promise<GradoAcademico> => {
  return await invoke('reactivar_grado', { idGrado: id_grado });
};

// Reporte API
export const getEstadisticasProyectosXDocente = async (): Promise<DocenteProyectosCount[]> => {
  return await invoke('get_estadisticas_proyectos_x_docente');
};

export const getKpisDashboard = async (): Promise<KpisDashboard> => {
  return await invoke('get_kpis_dashboard');
};

export const getDataExportacionPlana = async (): Promise<ExportData[]> => {
  return await invoke('get_data_exportacion_plana');
};

// NEW: Enhanced export grouped by docente
export interface DatosExportDocenteAgrupado {
  docente: string;
  dni: string;
  grado: string;
  cantidad_proyectos: number;
  proyectos: string | null;
}

export const getDataExportacionAgrupada = async (): Promise<DatosExportDocenteAgrupado[]> => {
  return await invoke('get_data_exportacion_agrupada_docente');
};

export interface Usuario {
  id_usuario: string;
  username: string;
  nombre_completo: string;
  rol: 'admin' | 'operador' | 'consulta' | string;
  activo: number;
}

export interface AuthStatus {
  has_users: boolean;
  requires_setup: boolean;
}

export const getAuthStatus = async (): Promise<AuthStatus> => {
  return await invoke('get_auth_status');
};

export const registrarPrimerUsuario = async (
  username: string,
  nombre_completo: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('registrar_primer_usuario', { request: { username, nombre_completo, password } });
};

export const loginUsuario = async (
  username: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('login_usuario', { request: { username, password } });
};

export const crearUsuario = async (
  username: string,
  nombre_completo: string,
  rol: string,
  password: string,
): Promise<Usuario> => {
  return await invoke('crear_usuario', { request: { username, nombre_completo, rol, password } });
};

export const getAllUsuarios = async (): Promise<Usuario[]> => {
  return await invoke('get_all_usuarios');
};

export const actualizarUsuario = async (
  id_usuario: string,
  username: string,
  nombre_completo: string,
  rol: string,
  password?: string,
): Promise<Usuario> => {
  return await invoke('actualizar_usuario', {
    idUsuario: id_usuario,
    request: { username, nombre_completo, rol, password: password?.trim() ? password : null },
  });
};

export const desactivarUsuario = async (id_usuario: string): Promise<Usuario> => {
  return await invoke('desactivar_usuario', { idUsuario: id_usuario });
};

export const reactivarUsuario = async (id_usuario: string): Promise<Usuario> => {
  return await invoke('reactivar_usuario', { idUsuario: id_usuario });
};