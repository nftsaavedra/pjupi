use chrono::NaiveDate;
use serde::Deserialize;

use crate::config::RenacytConfig;
use crate::domain::docente::RenacytLookupResult;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct RenacytPostulanteEnvelope {
    #[serde(default)]
    responseCode: String,
    #[serde(default)]
    data: Option<RenacytPostulanteData>,
    #[serde(default)]
    messageErrors: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct RenacytPostulanteData {
    #[serde(default)]
    idInvestigador: String,
    #[serde(default)]
    idOrcid: String,
    #[serde(default)]
    idPerfilScopus: String,
    #[serde(default)]
    nroDocumento: String,
    #[serde(default)]
    nombreCompleto: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct RenacytActoRegistralData {
    #[serde(default)]
    codigoRegistro: String,
    #[serde(default)]
    numeroDocumento: String,
    #[serde(default)]
    orcid: String,
    #[serde(default)]
    ctiVitae: String,
    #[serde(default)]
    grupo: String,
    #[serde(default)]
    nivel: String,
    #[serde(default)]
    condicion: String,
    #[serde(default)]
    fechaRegistroActivo: Option<i64>,
}

pub async fn consultar_investigador(config: &RenacytConfig, codigo_o_id: &str) -> Result<RenacytLookupResult, AppError> {
    let id_investigador = normalize_id_investigador(codigo_o_id)?;
    let client = reqwest::Client::new();

    let postulante_url = format!(
        "{}/postulante/obtenerDatosPostulante/{}",
        config.api_base_url.trim_end_matches('/'),
        id_investigador,
    );
    let acto_url = format!(
        "{}/actoRegistral/obtenerActoRegistralActivoCtiVitae/{}/{}",
        config.api_base_url.trim_end_matches('/'),
        config.acto_version.trim(),
        id_investigador,
    );

    let (postulante_response, acto_response) = tokio::try_join!(
        client.get(&postulante_url).send(),
        client.get(&acto_url).send(),
    )?;

    if !postulante_response.status().is_success() {
        return Err(AppError::ExternalServiceError(format!(
            "La consulta RENACYT del postulante no pudo completarse ({})",
            postulante_response.status()
        )));
    }

    if !acto_response.status().is_success() {
        return Err(AppError::ExternalServiceError(format!(
            "La consulta RENACYT del acto registral no pudo completarse ({})",
            acto_response.status()
        )));
    }

    let postulante_payload = postulante_response.json::<RenacytPostulanteEnvelope>().await?;
    let postulante = postulante_payload.data.ok_or_else(|| {
        AppError::ExternalServiceError(if postulante_payload.messageErrors.trim().is_empty() {
            "RENACYT no devolvió datos del investigador consultado.".to_string()
        } else {
            postulante_payload.messageErrors
        })
    })?;

    if postulante_payload.responseCode.trim() != "1" {
        return Err(AppError::ExternalServiceError(
            "RENACYT devolvió una respuesta no válida para el investigador consultado.".to_string(),
        ));
    }

    let acto = acto_response.json::<RenacytActoRegistralData>().await?;
    let ficha_url = build_ficha_url(config, &id_investigador);
    let ficha_html = client.get(&ficha_url).send().await?.text().await?;

    Ok(RenacytLookupResult {
        codigo_registro: first_non_empty(&[&acto.codigoRegistro, &build_codigo_registro(&id_investigador)]).unwrap_or_default(),
        id_investigador: first_non_empty(&[&acto.ctiVitae, &postulante.idInvestigador, &id_investigador]).unwrap_or_default(),
        nombre_completo: non_empty(postulante.nombreCompleto),
        numero_documento: first_non_empty_owned(vec![acto.numeroDocumento, postulante.nroDocumento]),
        nivel: non_empty(acto.nivel),
        grupo: non_empty(acto.grupo),
        condicion: non_empty(acto.condicion),
        fecha_informe_calificacion: extract_date_value(&ficha_html, "Fecha de informe de calificación :"),
        fecha_registro: acto.fechaRegistroActivo,
        fecha_ultima_revision: extract_date_value(&ficha_html, "Fecha de última revisión :"),
        orcid: first_non_empty_owned(vec![acto.orcid, postulante.idOrcid]),
        scopus_author_id: non_empty(postulante.idPerfilScopus),
        ficha_url,
    })
}

fn normalize_id_investigador(value: &str) -> Result<String, AppError> {
    let cleaned = value.trim().to_uppercase();
    if cleaned.is_empty() {
        return Err(AppError::ExternalServiceError(
            "Ingrese un código RENACYT o ID de investigador válido.".to_string(),
        ));
    }

    let numeric = if let Some(code) = cleaned.strip_prefix('P') {
        let digits = code.trim_start_matches('0');
        if digits.is_empty() { "0".to_string() } else { digits.to_string() }
    } else {
        cleaned
    };

    if !numeric.chars().all(|character| character.is_ascii_digit()) {
        return Err(AppError::ExternalServiceError(
            "El código RENACYT o ID de investigador solo debe contener valores numéricos válidos.".to_string(),
        ));
    }

    Ok(numeric)
}

fn build_codigo_registro(id_investigador: &str) -> String {
    let id_num = id_investigador.parse::<u64>().unwrap_or_default();
    format!("P{id_num:07}")
}

fn build_ficha_url(config: &RenacytConfig, id_investigador: &str) -> String {
    format!("{}?idInvestigador={}", config.ficha_base_url.trim_end_matches('/'), id_investigador)
}

fn extract_date_value(html: &str, label: &str) -> Option<i64> {
    let start = html.find(label)? + label.len();
    let tail = &html[start..];
    let mut value = String::new();

    for character in tail.chars() {
        if character == '•' || character == '<' || character == '\n' || character == '\r' {
            break;
        }
        value.push(character);
    }

    let normalized = value.replace("&nbsp;", " ").trim().to_string();
    if normalized.is_empty() {
        return None;
    }

    NaiveDate::parse_from_str(&normalized, "%d/%m/%Y")
        .ok()
        .and_then(|date| date.and_hms_opt(0, 0, 0))
        .map(|date_time| date_time.and_utc().timestamp_millis())
}

fn non_empty(value: String) -> Option<String> {
    let normalized = value.trim().to_string();
    (!normalized.is_empty()).then_some(normalized)
}

fn first_non_empty(values: &[&str]) -> Option<String> {
    values
        .iter()
        .map(|value| value.trim())
        .find(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn first_non_empty_owned(values: Vec<String>) -> Option<String> {
    values
        .into_iter()
        .map(|value| value.trim().to_string())
        .find(|value| !value.is_empty())
}