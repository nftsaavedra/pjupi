use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDocenteRenacytRequest {
    pub codigo_registro: String,
    pub id_investigador: String,
    pub nivel: Option<String>,
    pub grupo: Option<String>,
    pub condicion: Option<String>,
    pub fecha_informe_calificacion: Option<i64>,
    pub fecha_registro: Option<i64>,
    pub fecha_ultima_revision: Option<i64>,
    pub orcid: Option<String>,
    pub scopus_author_id: Option<String>,
    pub ficha_url: String,
    pub formaciones_academicas_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenacytLookupResult {
    pub codigo_registro: String,
    pub id_investigador: String,
    pub nombre_completo: Option<String>,
    pub numero_documento: Option<String>,
    pub nivel: Option<String>,
    pub grupo: Option<String>,
    pub condicion: Option<String>,
    pub fecha_informe_calificacion: Option<i64>,
    pub fecha_registro: Option<i64>,
    pub fecha_ultima_revision: Option<i64>,
    pub orcid: Option<String>,
    pub scopus_author_id: Option<String>,
    pub ficha_url: String,
    pub solicitud_id: Option<i64>,
    pub formaciones_academicas_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Docente {
    pub id_docente: String,
    pub dni: String,
    pub id_grado: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub activo: i64,
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
    pub renacyt_fecha_ultima_sincronizacion: Option<i64>,
    pub renacyt_ficha_url: Option<String>,
    pub renacyt_formaciones_academicas_json: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocenteRequest {
    pub dni: String,
    pub id_grado: String,
    pub nombres: String,
    pub apellido_paterno: String,
    pub apellido_materno: Option<String>,
    pub renacyt: Option<CreateDocenteRenacytRequest>,
}

// New: Docente with detail data for reports
#[derive(Debug, Serialize, FromRow)]
pub struct DocenteDetalle {
    pub id_docente: String,
    pub dni: String,
    pub nombres_apellidos: String,
    pub nombres: Option<String>,
    pub apellido_paterno: Option<String>,
    pub apellido_materno: Option<String>,
    pub grado: String,
    pub cantidad_proyectos: i64,
    pub proyectos: Option<String>, // JSON array of project titles, comma-separated
    pub activo: i64,
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
    pub renacyt_fecha_ultima_sincronizacion: Option<i64>,
    pub renacyt_ficha_url: Option<String>,
    pub renacyt_formaciones_academicas_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReniecDniLookupResult {
    pub first_name: String,
    pub first_last_name: String,
    pub second_last_name: String,
    pub full_name: String,
    pub document_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EliminarDocenteResultado {
    pub accion: String, // "desactivado"
    pub mensaje: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshDocenteRenacytFormacionResultado {
    pub docente: DocenteDetalle,
    pub actualizada: bool,
    pub mensaje: String,
}

impl Docente {
    pub fn new(request: CreateDocenteRequest) -> Self {
        let apellido_materno = request
            .apellido_materno
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let nombres = request.nombres.trim().to_string();
        let apellido_paterno = request.apellido_paterno.trim().to_string();
        let nombres_apellidos = [
            Some(nombres.clone()),
            Some(apellido_paterno.clone()),
            apellido_materno.clone(),
        ]
        .into_iter()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");
        let renacyt = request.renacyt;
        let fecha_ultima_sincronizacion = renacyt.as_ref().map(|_| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_millis() as i64)
                .unwrap_or_default()
        });

        Self {
            id_docente: Uuid::new_v4().to_string(),
            dni: request.dni,
            id_grado: request.id_grado,
            nombres_apellidos,
            nombres: Some(nombres),
            apellido_paterno: Some(apellido_paterno),
            apellido_materno,
            activo: 1,
            renacyt_codigo_registro: renacyt.as_ref().map(|value| value.codigo_registro.trim().to_string()).filter(|value| !value.is_empty()),
            renacyt_id_investigador: renacyt.as_ref().map(|value| value.id_investigador.trim().to_string()).filter(|value| !value.is_empty()),
            renacyt_nivel: renacyt.as_ref().and_then(|value| value.nivel.clone()).filter(|value| !value.trim().is_empty()),
            renacyt_grupo: renacyt.as_ref().and_then(|value| value.grupo.clone()).filter(|value| !value.trim().is_empty()),
            renacyt_condicion: renacyt.as_ref().and_then(|value| value.condicion.clone()).filter(|value| !value.trim().is_empty()),
            renacyt_fecha_informe_calificacion: renacyt.as_ref().and_then(|value| value.fecha_informe_calificacion),
            renacyt_fecha_registro: renacyt.as_ref().and_then(|value| value.fecha_registro),
            renacyt_fecha_ultima_revision: renacyt.as_ref().and_then(|value| value.fecha_ultima_revision),
            renacyt_orcid: renacyt.as_ref().and_then(|value| value.orcid.clone()).filter(|value| !value.trim().is_empty()),
            renacyt_scopus_author_id: renacyt.as_ref().and_then(|value| value.scopus_author_id.clone()).filter(|value| !value.trim().is_empty()),
            renacyt_fecha_ultima_sincronizacion: fecha_ultima_sincronizacion,
            renacyt_ficha_url: renacyt.as_ref().map(|value| value.ficha_url.trim().to_string()).filter(|value| !value.is_empty()),
            renacyt_formaciones_academicas_json: renacyt.as_ref().and_then(|value| value.formaciones_academicas_json.clone()).filter(|value| !value.trim().is_empty()),
        }
    }

    pub fn apply_renacyt_refresh(&mut self, lookup: RenacytLookupResult) -> bool {
        let nuevas_formaciones = lookup
            .formaciones_academicas_json
            .filter(|value| !value.trim().is_empty());
        let tiene_nuevas_formaciones = nuevas_formaciones.is_some();

        self.renacyt_codigo_registro = Some(lookup.codigo_registro.trim().to_string()).filter(|value| !value.is_empty());
        self.renacyt_id_investigador = Some(lookup.id_investigador.trim().to_string()).filter(|value| !value.is_empty());
        self.renacyt_nivel = lookup.nivel.filter(|value| !value.trim().is_empty());
        self.renacyt_grupo = lookup.grupo.filter(|value| !value.trim().is_empty());
        self.renacyt_condicion = lookup.condicion.filter(|value| !value.trim().is_empty());
        self.renacyt_fecha_informe_calificacion = lookup.fecha_informe_calificacion;
        self.renacyt_fecha_registro = lookup.fecha_registro;
        self.renacyt_fecha_ultima_revision = lookup.fecha_ultima_revision;
        self.renacyt_orcid = lookup.orcid.filter(|value| !value.trim().is_empty());
        self.renacyt_scopus_author_id = lookup.scopus_author_id.filter(|value| !value.trim().is_empty());
        self.renacyt_ficha_url = Some(lookup.ficha_url.trim().to_string()).filter(|value| !value.is_empty());
        self.renacyt_fecha_ultima_sincronizacion = Some(Self::current_timestamp_ms());

        if let Some(formaciones) = nuevas_formaciones {
            self.renacyt_formaciones_academicas_json = Some(formaciones);
        }

        tiene_nuevas_formaciones
    }

    fn current_timestamp_ms() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .unwrap_or_default()
    }
}