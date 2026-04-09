export interface Docente {
  id_docente: string;
  dni: string;
  id_grado: string;
  nombres_apellidos: string;
  nombres?: string | null;
  apellido_paterno?: string | null;
  apellido_materno?: string | null;
  activo?: number;
  renacyt_codigo_registro?: string | null;
  renacyt_id_investigador?: string | null;
  renacyt_nivel?: string | null;
  renacyt_grupo?: string | null;
  renacyt_condicion?: string | null;
  renacyt_fecha_informe_calificacion?: number | null;
  renacyt_fecha_registro?: number | null;
  renacyt_fecha_ultima_revision?: number | null;
  renacyt_orcid?: string | null;
  renacyt_scopus_author_id?: string | null;
  renacyt_fecha_ultima_sincronizacion?: number | null;
  renacyt_ficha_url?: string | null;
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
  renacyt_codigo_registro?: string | null;
  renacyt_id_investigador?: string | null;
  renacyt_nivel?: string | null;
  renacyt_grupo?: string | null;
  renacyt_condicion?: string | null;
  renacyt_fecha_informe_calificacion?: number | null;
  renacyt_fecha_registro?: number | null;
  renacyt_fecha_ultima_revision?: number | null;
  renacyt_orcid?: string | null;
  renacyt_scopus_author_id?: string | null;
  renacyt_fecha_ultima_sincronizacion?: number | null;
  renacyt_ficha_url?: string | null;
}

export interface RenacytLookupResult {
  codigo_registro: string;
  id_investigador: string;
  nombre_completo?: string | null;
  numero_documento?: string | null;
  nivel?: string | null;
  grupo?: string | null;
  condicion?: string | null;
  fecha_informe_calificacion?: number | null;
  fecha_registro?: number | null;
  fecha_ultima_revision?: number | null;
  orcid?: string | null;
  scopus_author_id?: string | null;
  ficha_url: string;
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