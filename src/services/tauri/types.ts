export interface Docente {
  id_docente: string;
  dni: string;
  id_grado: string;
  nombres_apellidos: string;
  nombres?: string | null;
  apellido_paterno?: string | null;
  apellido_materno?: string | null;
  activo?: number;
}

export interface DocenteDetalle {
  id_docente: string;
  dni: string;
  nombres_apellidos: string;
  nombres?: string | null;
  apellido_paterno?: string | null;
  apellido_materno?: string | null;
  grado: string;
  cantidad_proyectos: number;
  proyectos: string | null;
  activo: number;
}

export interface ReniecDniLookupResult {
  first_name: string;
  first_last_name: string;
  second_last_name: string;
  full_name: string;
  document_number: string;
}

export interface EliminarDocenteResultado {
  accion: 'desactivado' | string;
  mensaje: string;
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

export interface DatosExportDocenteAgrupado {
  docente: string;
  dni: string;
  grado: string;
  cantidad_proyectos: number;
  proyectos: string | null;
}

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