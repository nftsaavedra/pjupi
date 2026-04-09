use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::config::ReniecConfig;
use crate::domain::docente::ReniecDniLookupResult;
use crate::error::AppError;

pub async fn consultar_dni(config: &ReniecConfig, numero: &str) -> Result<ReniecDniLookupResult, AppError> {
    let numero_limpio = numero.trim();
    if !numero_limpio.chars().all(|char| char.is_ascii_digit()) || numero_limpio.len() != 8 {
        return Err(AppError::ExternalServiceError(
            "El DNI debe tener exactamente 8 dígitos numéricos.".to_string(),
        ));
    }

    let token = config.token.as_ref().ok_or_else(|| {
        AppError::ConfigurationError(
            "La integración RENIEC no está configurada. Defina PJUPI_RENIEC_TOKEN en el archivo .env.".to_string(),
        )
    })?;

    let endpoint = format!("{}/reniec/dni", config.api_base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .query(&[("numero", numero_limpio)])
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await?;

    let status = response.status();
    if status.is_success() {
        return response.json::<ReniecDniLookupResult>().await.map_err(Into::into);
    }

    if status.as_u16() == 400 || status.as_u16() == 404 {
        return Err(AppError::ExternalServiceError(
            "No se encontraron datos válidos para el DNI consultado en RENIEC.".to_string(),
        ));
    }

    let detalle = response.text().await.unwrap_or_else(|_| "Sin detalle adicional".to_string());
    Err(AppError::ExternalServiceError(format!(
        "La consulta RENIEC no pudo completarse en este momento ({status}). {detalle}"
    )))
}