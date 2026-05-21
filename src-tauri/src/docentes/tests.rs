#[cfg(test)]
mod tests {
    use crate::docentes::models::{CreateDocenteRequest, CreateDocenteRenacytRequest, Docente};

    #[test]
    fn test_docente_new_builds_full_name_correctly() {
        let request = CreateDocenteRequest {
            dni: "12345678".to_string(),
            id_grado: "grado-1".to_string(),
            nombres: "Juan Carlos".to_string(),
            apellido_paterno: "Pérez".to_string(),
            apellido_materno: Some("García".to_string()),
            renacyt: None,
        };

        let docente = Docente::new(request);

        assert_eq!(docente.dni, "12345678");
        assert_eq!(docente.nombres_apellidos, "Juan Carlos Pérez García");
        assert_eq!(docente.activo, 1);
        assert!(docente.id_docente.len() > 0);
        assert!(docente.updated_at.is_some());
    }

    #[test]
    fn test_docente_new_without_apellido_materno() {
        let request = CreateDocenteRequest {
            dni: "87654321".to_string(),
            id_grado: "grado-2".to_string(),
            nombres: "María".to_string(),
            apellido_paterno: "López".to_string(),
            apellido_materno: None,
            renacyt: None,
        };

        let docente = Docente::new(request);
        assert_eq!(docente.nombres_apellidos, "María López");
    }

    #[test]
    fn test_docente_new_with_renacyt_data() {
        let renacyt = CreateDocenteRenacytRequest {
            codigo_registro: "P12345".to_string(),
            id_investigador: "INV-001".to_string(),
            nivel: Some("VI".to_string()),
            grupo: Some("A".to_string()),
            condicion: Some("Activo".to_string()),
            fecha_informe_calificacion: None,
            fecha_registro: None,
            fecha_ultima_revision: None,
            orcid: Some("0000-0001-2345-6789".to_string()),
            scopus_author_id: Some("56789012345".to_string()),
            ficha_url: "https://ficha.renacyt/12345".to_string(),
            formaciones_academicas_json: None,
        };

        let request = CreateDocenteRequest {
            dni: "11223344".to_string(),
            id_grado: "grado-3".to_string(),
            nombres: "Ana".to_string(),
            apellido_paterno: "Martínez".to_string(),
            apellido_materno: None,
            renacyt: Some(renacyt),
        };

        let docente = Docente::new(request);
        assert_eq!(docente.renacyt_codigo_registro, Some("P12345".to_string()));
        assert_eq!(docente.renacyt_nivel, Some("VI".to_string()));
        assert!(docente.renacyt_fecha_ultima_sincronizacion.is_some());
    }

    #[test]
    fn test_docente_apply_renacyt_refresh_updates_fields() {
        let request = CreateDocenteRequest {
            dni: "55555555".to_string(),
            id_grado: "grado-4".to_string(),
            nombres: "Test".to_string(),
            apellido_paterno: "User".to_string(),
            apellido_materno: None,
            renacyt: None,
        };
        let mut docente = Docente::new(request);

        let lookup = crate::docentes::models::RenacytLookupResult {
            codigo_registro: "P99999".to_string(),
            id_investigador: "INV-999".to_string(),
            nombre_completo: Some("Test User".to_string()),
            numero_documento: None,
            nivel: Some("I".to_string()),
            grupo: Some("B".to_string()),
            condicion: Some("Activo".to_string()),
            fecha_informe_calificacion: None,
            fecha_registro: None,
            fecha_ultima_revision: None,
            orcid: Some("0000-0002-0000-0000".to_string()),
            scopus_author_id: None,
            ficha_url: "https://ficha.renacyt/99999".to_string(),
            solicitud_id: None,
            formaciones_academicas_json: Some(r#"[{"grado":"Doctor"}]"#.to_string()),
        };

        let updated = docente.apply_renacyt_refresh(lookup);
        assert!(updated, "Debe reportar que hubo actualización");
        assert_eq!(docente.renacyt_nivel, Some("I".to_string()));
        assert_eq!(docente.renacyt_grupo, Some("B".to_string()));
        assert_eq!(docente.renacyt_orcid, Some("0000-0002-0000-0000".to_string()));
        assert!(docente.renacyt_formaciones_academicas_json.is_some());
    }
}
