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
  renacyt_formaciones_academicas_json?: string | null;
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
  renacyt_formaciones_academicas_json?: string | null;
}

export interface RenacytFormacionAcademicaResumen {
  id: number;
  centro_estudios?: string | null;
  grado_academico?: string | null;
  titulo?: string | null;
  fecha_inicio?: number | null;
  fecha_fin?: number | null;
  indicador_importado: boolean;
  puntaje_obtenido?: number | null;
  considerado_para_cc: boolean;
  es_calificado: boolean;
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
  solicitud_id?: number | null;
  formaciones_academicas_json?: string | null;
}

export interface ReniecDniLookupResult {
  first_name: string;
  first_last_name: string;
  second_last_name: string;
  full_name: string;
  document_number: string;
}

export interface EliminarDocenteResultado {
  accion: string;
  mensaje: string;
}

export interface RefreshDocenteRenacytFormacionResultado {
  docente: DocenteDetalle;
  actualizada: boolean;
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
  docente_responsable?: string | null;
  docentes: string | null;
  participantes_json?: string | null;
  activo: number;
}

export interface ProyectoParticipanteResumen {
  id_docente: string;
  nombre: string;
  grado: string;
  renacyt_nivel: string;
  es_responsable: boolean;
}

export interface EliminarProyectoResultado {
  accion: string;
  mensaje: string;
}

export interface GradoAcademico {
  id_grado: string;
  nombre: string;
  descripcion?: string;
  activo?: number;
}

export interface EliminarGradoResultado {
  accion: string;
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
  renacyt_nivel: string;
  docente: string;
  dni: string;
}

export interface DatosExportDocenteAgrupado {
  docente: string;
  dni: string;
  grado: string;
  renacyt_nivel: string;
  cantidad_proyectos: number;
  proyectos: string | null;
}

export interface Usuario {
  id_usuario: string;
  username: string;
  nombre_completo: string;
  rol: string;
  activo: number;
}

export interface AuthStatus {
  has_users: boolean;
  requires_setup: boolean;
}

// ─── PeruCRIS / Pure types ────────────────────────────────────────────────────

export interface Publicacion {
  id_publicacion: string;
  pure_uuid: string;
  docente_id: string;
  proyecto_id?: string | null;
  titulo: string;
  tipo_publicacion?: string | null;
  doi?: string | null;
  scopus_eid?: string | null;
  anio_publicacion?: number | null;
  autores_json?: string | null;
  estado_publicacion?: string | null;
  journal_titulo?: string | null;
  issn?: string | null;
  pure_sincronizado_at?: number | null;
  created_at?: number | null;
  updated_at?: number | null;
}

export interface SyncPublicacionesResult {
  docente_id: string;
  scopus_author_id: string;
  pure_person_uuid?: string | null;
  total_encontradas: number;
  nuevas: number;
  actualizadas: number;
}

export interface Patente {
  id_patente: string;
  proyecto_id?: string | null;
  docente_id?: string | null;
  titulo: string;
  numero_patente?: string | null;
  tipo?: string | null;
  fecha_solicitud?: number | null;
  fecha_concesion?: number | null;
  pais?: string | null;
  entidad_concedente?: string | null;
  descripcion?: string | null;
  created_at?: number | null;
  updated_at?: number | null;
}

export interface Producto {
  id_producto: string;
  proyecto_id?: string | null;
  docente_id?: string | null;
  nombre: string;
  tipo?: string | null;
  descripcion?: string | null;
  fecha_registro?: number | null;
  created_at?: number | null;
  updated_at?: number | null;
}

export interface Equipamiento {
  id_equipamiento: string;
  proyecto_id?: string | null;
  nombre: string;
  descripcion?: string | null;
  valor_estimado?: number | null;
  moneda?: string | null;
  proveedor?: string | null;
  fecha_adquisicion?: number | null;
  created_at?: number | null;
  updated_at?: number | null;
}

export interface Financiamiento {
  id_financiamiento: string;
  proyecto_id?: string | null;
  entidad_financiadora: string;
  tipo?: string | null;
  monto?: number | null;
  moneda?: string | null;
  fecha_inicio?: number | null;
  fecha_fin?: number | null;
  descripcion?: string | null;
  created_at?: number | null;
  updated_at?: number | null;
}

export interface GrupoInvestigacion {
  id_grupo: string;
  nombre: string;
  descripcion?: string | null;
  coordinador_id?: string | null;
  lineas_investigacion: string[];
  activo: number;
  created_at?: number | null;
  updated_at?: number | null;
}