use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SecurityStatus {
    pub database_backend: String,
    pub mongodb_configured: bool,
    pub security_recommendations: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ConfigurationGuide {
    pub title: String,
    pub steps: Vec<ConfigurationStep>,
}

#[derive(Serialize, Debug)]
pub struct ConfigurationStep {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub example: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SecurityRecommendations {
    pub recommendations: Vec<SecurityRecommendation>,
}

#[derive(Serialize, Debug)]
pub struct SecurityRecommendation {
    pub category: String,
    pub title: String,
    pub description: String,
    pub priority: String,
}
