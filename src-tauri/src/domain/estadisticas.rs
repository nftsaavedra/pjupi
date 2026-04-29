use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocenteProyectosCount {
    pub nombre: String,
    pub cantidad: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KpisDashboard {
    // Counts only active entities.
    pub total_proyectos: i64,
    pub total_docentes: i64,
    pub docentes_con_1_proyecto: i64,
    pub docentes_multiples_proyectos: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub proyecto: String,
    pub grado: String,
    pub renacyt_nivel: String,
    pub docente: String,
    pub dni: String,
}