// src/models.rs (Código Actualizado para Reporte Detallado)

use serde::{Deserialize, Serialize};

// --- Tipos de Datos Categóricos ---
#[derive(Debug, Deserialize, Serialize)]
pub enum AssetType {
    RealEstate,
    IP, 
    Commodity,
    Service,
    Equity,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MaturityStage {
    Idea,
    MVP,
    Revenue,
    Scale,
}

// --- ESTRUCTURA DE DETALLE DE REGLA (NUEVA) ---
// Representa el resultado individual de cada una de las 7 reglas.
#[derive(Debug, Serialize)]
pub struct RuleDetail {
    pub rule_name: String,
    pub description: String,
    pub passed: bool,
    pub weight: f64, // Ponderación de la regla
    pub contribution_score: f64, // Puntuación aportada (weight si passed=true, 0 si passed=false)
    pub rationale: String, // Explicación de por qué pasó o falló
}


// --- ESTRUCTURA PRINCIPAL DE DATOS DE ENTRADA (MAPPING del Cuestionario) ---
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectSubmission {
    pub project_name: String,
    pub description: String, 
    pub asset_type: AssetType, 
    pub market_size_usd: u64, 
    pub capital_needed_usd: u64, 
    pub expected_roi_percent: f64, 
    pub regulatory_risk_score: u8, 
    pub jurisdiction: String, 
    pub team_experience_years: u8, 
    pub has_database_capability: bool, 
    pub maturity_stage: MaturityStage, 
}

// --- ESTRUCTURA DE RESULTADO DE EVALUACIÓN (Output) ---
#[derive(Debug, Serialize)]
pub struct EvaluationResult {
    pub is_viable: bool,
    pub viability_score: f64,
    // CAMBIO CLAVE: Reemplazamos 'failed_rules' por el reporte detallado
    pub rule_details: Vec<RuleDetail>, 
    pub report_link: String, 
}
