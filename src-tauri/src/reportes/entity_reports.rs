use serde::Serialize;

// ═══════════════════════════════════════════════════════════════════════════════
// ReporteProyectoIntegral — Scenario: "Un proyecto, TODA su información"
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct ReporteProyectoIntegral {
    pub cabecera: ProyectoCabeceraReporte,
    pub equipo: Vec<MiembroProyectoReporte>,
    pub total_docentes: usize,
    pub patentes: Vec<PatenteConEtiquetas>,
    pub total_patentes: usize,
    pub productos: Vec<ProductoConEtiquetas>,
    pub total_productos: usize,
    pub equipamientos: Vec<EquipamientoConEtiquetas>,
    pub total_equipamientos: usize,
    pub financiamientos: Vec<FinanciamientoConEtiquetas>,
    pub total_financiamientos: usize,
    pub resumen_financiero: ResumenFinanciero,
}

#[derive(Debug, Serialize)]
pub struct ProyectoCabeceraReporte {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub activo: bool,
    pub campo_ocde: Option<String>,
    pub programas_relacionados: Vec<String>,
    pub fecha_creacion: Option<String>,
    pub fecha_actualizacion: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MiembroProyectoReporte {
    pub id_docente: String,
    pub dni: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub grado_nombre: String,
    pub grado_id: String,
    pub es_responsable: bool,
    pub renacyt_codigo_registro: Option<String>,
    pub renacyt_nivel: Option<String>,
    pub renacyt_grupo: Option<String>,
    pub renacyt_condicion: Option<String>,
    pub renacyt_orcid: Option<String>,
    pub renacyt_scopus_author_id: Option<String>,
    pub grupo_nombre: Option<String>,
    pub grupo_id: Option<String>,
    pub publicaciones_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PatenteConEtiquetas {
    pub id_patente: String,
    pub titulo: String,
    pub numero_patente: Option<String>,
    pub tipo_codigo: Option<String>,
    pub tipo_nombre: Option<String>,
    pub estado_codigo: Option<String>,
    pub estado_nombre: Option<String>,
    pub fecha_solicitud: Option<i64>,
    pub fecha_concesion: Option<i64>,
    pub pais: Option<String>,
    pub entidad_concedente: Option<String>,
    pub descripcion: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductoConEtiquetas {
    pub id_producto: String,
    pub nombre: String,
    pub tipo_codigo: Option<String>,
    pub tipo_nombre: Option<String>,
    pub etapa_codigo: Option<String>,
    pub etapa_nombre: Option<String>,
    pub descripcion: Option<String>,
    pub fecha_registro: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EquipamientoConEtiquetas {
    pub id_equipamiento: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub especificaciones: Option<String>,
    pub valor_estimado: Option<f64>,
    pub moneda_codigo: Option<String>,
    pub moneda_nombre: Option<String>,
    pub proveedor: Option<String>,
    pub fecha_adquisicion: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct FinanciamientoConEtiquetas {
    pub id_financiamiento: String,
    pub entidad_financiadora: String,
    pub tipo_codigo: Option<String>,
    pub tipo_nombre: Option<String>,
    pub monto: Option<f64>,
    pub moneda_codigo: Option<String>,
    pub moneda_nombre: Option<String>,
    pub fecha_inicio: Option<i64>,
    pub fecha_fin: Option<i64>,
    pub descripcion: Option<String>,
    pub estado_financiero_codigo: Option<String>,
    pub estado_financiero_nombre: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResumenFinanciero {
    pub total_financiamientos: usize,
    pub desglose_por_moneda: Vec<MonedaDesglose>,
    pub desglose_por_estado: Vec<EstadoDesglose>,
}

#[derive(Debug, Serialize)]
pub struct MonedaDesglose {
    pub moneda_codigo: String,
    pub moneda_nombre: String,
    pub cantidad: usize,
    pub monto_total: f64,
}

#[derive(Debug, Serialize)]
pub struct EstadoDesglose {
    pub estado_codigo: String,
    pub estado_nombre: String,
    pub cantidad: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ReporteDocenteIntegral — Scenario: "Un docente, TODA su información"
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct ReporteDocenteIntegral {
    pub perfil: PerfilDocenteReporte,
    pub proyectos: Vec<ProyectoDocenteDetalle>,
    pub total_proyectos: usize,
    pub recursos: RecursosDocenteResumen,
    pub publicaciones: Vec<PublicacionConEtiquetas>,
    pub total_publicaciones: usize,
    pub trazabilidad: TrazabilidadDocente,
}

#[derive(Debug, Serialize)]
pub struct PerfilDocenteReporte {
    pub id_docente: String,
    pub dni: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub grado_nombre: String,
    pub grado_id: String,
    pub renacyt_codigo_registro: Option<String>,
    pub renacyt_id_investigador: Option<String>,
    pub renacyt_nivel: Option<String>,
    pub renacyt_grupo: Option<String>,
    pub renacyt_condicion: Option<String>,
    pub renacyt_fecha_informe_calificacion: Option<i64>,
    pub renacyt_fecha_registro: Option<i64>,
    pub renacyt_fecha_ultima_revision: Option<i64>,
    pub renacyt_orcid: Option<String>,
    pub renacyt_scopus_author_id: Option<String>,
    pub renacyt_ficha_url: Option<String>,
    pub renacyt_formaciones_academicas_json: Option<String>,
    pub grupo_nombre: Option<String>,
    pub grupo_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProyectoDocenteDetalle {
    pub id_proyecto: String,
    pub titulo_proyecto: String,
    pub es_responsable: bool,
    pub activo: bool,
    pub campo_ocde: Option<String>,
    pub programas_relacionados: Vec<String>,
    pub colegas: Vec<ColegaProyecto>,
    pub recursos_en_proyecto: RecursosProyectoResumen,
}

#[derive(Debug, Serialize)]
pub struct ColegaProyecto {
    pub id_docente: String,
    pub nombres_apellidos: String,
    pub grado_nombre: String,
    pub es_responsable: bool,
}

#[derive(Debug, Serialize)]
pub struct RecursosProyectoResumen {
    pub patentes: usize,
    pub productos: usize,
    pub equipamientos: usize,
    pub financiamientos: usize,
}

#[derive(Debug, Serialize)]
pub struct RecursosDocenteResumen {
    pub patentes: Vec<PatenteConEtiquetas>,
    pub productos: Vec<ProductoConEtiquetas>,
    pub equipamientos: Vec<EquipamientoConEtiquetas>,
    pub total_patentes: usize,
    pub total_productos: usize,
    pub total_equipamientos: usize,
}

#[derive(Debug, Serialize)]
pub struct TrazabilidadDocente {
    pub updated_at: Option<i64>,
    pub fecha_ultima_sincronizacion_renacyt: Option<i64>,
    pub fecha_ultima_sincronizacion_pure: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PublicacionConEtiquetas {
    pub id_publicacion: String,
    pub pure_uuid: String,
    pub titulo: String,
    pub tipo_publicacion: Option<String>,
    pub doi: Option<String>,
    pub scopus_eid: Option<String>,
    pub anio_publicacion: Option<i32>,
    pub autores_json: Option<String>,
    pub estado_publicacion: Option<String>,
    pub journal_titulo: Option<String>,
    pub issn: Option<String>,
    pub pure_sincronizado_at: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

impl PatenteConEtiquetas {
    pub fn from_patente(p: &crate::recursos::models::Patente, catalogo_map: &std::collections::HashMap<(String, String), crate::catalogos::models::CatalogoItem>) -> Self {
        let tipo_lbl = p.tipo.as_ref().and_then(|c| catalogo_map.get(&("tipo_patente".to_string(), c.clone())).map(|i| i.nombre.clone()));
        let estado_lbl = p.estado.as_ref().and_then(|c| catalogo_map.get(&("estado_patente".to_string(), c.clone())).map(|i| i.nombre.clone()));
        Self { id_patente: p.id_patente.clone(), titulo: p.titulo.clone(), numero_patente: p.numero_patente.clone(), tipo_codigo: p.tipo.clone(), tipo_nombre: tipo_lbl, estado_codigo: p.estado.clone(), estado_nombre: estado_lbl, fecha_solicitud: p.fecha_solicitud, fecha_concesion: p.fecha_concesion, pais: p.pais.clone(), entidad_concedente: p.entidad_concedente.clone(), descripcion: p.descripcion.clone() }
    }
}

impl ProductoConEtiquetas {
    pub fn from_producto(p: &crate::recursos::models::Producto, catalogo_map: &std::collections::HashMap<(String, String), crate::catalogos::models::CatalogoItem>) -> Self {
        let tipo_lbl = p.tipo.as_ref().and_then(|c| catalogo_map.get(&("tipo_producto".to_string(), c.clone())).map(|i| i.nombre.clone()));
        let etapa_lbl = p.etapa.as_ref().and_then(|c| catalogo_map.get(&("etapa_producto".to_string(), c.clone())).map(|i| i.nombre.clone()));
        Self { id_producto: p.id_producto.clone(), nombre: p.nombre.clone(), tipo_codigo: p.tipo.clone(), tipo_nombre: tipo_lbl, etapa_codigo: p.etapa.clone(), etapa_nombre: etapa_lbl, descripcion: p.descripcion.clone(), fecha_registro: p.fecha_registro }
    }
}

impl EquipamientoConEtiquetas {
    pub fn from_equipamiento(e: &crate::recursos::models::Equipamiento, catalogo_map: &std::collections::HashMap<(String, String), crate::catalogos::models::CatalogoItem>) -> Self {
        let moneda_lbl = e.moneda.as_ref().and_then(|c| catalogo_map.get(&("moneda".to_string(), c.clone())).map(|i| i.nombre.clone()));
        Self { id_equipamiento: e.id_equipamiento.clone(), nombre: e.nombre.clone(), descripcion: e.descripcion.clone(), especificaciones: e.especificaciones.clone(), valor_estimado: e.valor_estimado, moneda_codigo: e.moneda.clone(), moneda_nombre: moneda_lbl, proveedor: e.proveedor.clone(), fecha_adquisicion: e.fecha_adquisicion }
    }
}

impl FinanciamientoConEtiquetas {
    pub fn from_financiamiento(f: &crate::recursos::models::Financiamiento, catalogo_map: &std::collections::HashMap<(String, String), crate::catalogos::models::CatalogoItem>) -> Self {
        let tipo_lbl = f.tipo.as_ref().and_then(|c| catalogo_map.get(&("tipo_financiamiento".to_string(), c.clone())).map(|i| i.nombre.clone()));
        let moneda_lbl = f.moneda.as_ref().and_then(|c| catalogo_map.get(&("moneda".to_string(), c.clone())).map(|i| i.nombre.clone()));
        let estado_lbl = f.estado_financiero.as_ref().and_then(|c| catalogo_map.get(&("estado_financiero".to_string(), c.clone())).map(|i| i.nombre.clone()));
        Self { id_financiamiento: f.id_financiamiento.clone(), entidad_financiadora: f.entidad_financiadora.clone(), tipo_codigo: f.tipo.clone(), tipo_nombre: tipo_lbl, monto: f.monto, moneda_codigo: f.moneda.clone(), moneda_nombre: moneda_lbl, fecha_inicio: f.fecha_inicio, fecha_fin: f.fecha_fin, descripcion: f.descripcion.clone(), estado_financiero_codigo: f.estado_financiero.clone(), estado_financiero_nombre: estado_lbl }
    }
}
