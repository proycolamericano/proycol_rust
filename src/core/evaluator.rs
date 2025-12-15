use serde::{Deserialize, Serialize};

// ====================================================================
// ESTRUCTURAS DE DATOS (Definidas aquí para evitar el error 'crate::models')
// ====================================================================

/// Tipos de Activos del Mundo Real (RWA) soportados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Certification,
    Property,
    Inventory,
}

/// Etapas de Madurez del Proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaturityStage {
    Idea,
    MVP,
    Scale,
}

/// Estructura para recibir los datos del frontend (Petición POST)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSubmission {
    pub project_name: String,
    pub requested_amount: f64,
    pub asset_type: AssetType,
    pub maturity_stage: MaturityStage,
    pub description: String,
}

/// Detalle de la evaluación de una regla específica
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleDetail {
    pub rule_name: String,
    pub compliance: bool,
    pub rationale: String,
    pub score_impact: f64,
}

/// Resultado final de la evaluación de consultoría
#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub project_name: String,
    pub status: String, // "Approved", "Conditionally Approved", "Rejected"
    pub total_score: f64,
    pub details: Vec<RuleDetail>,
    pub next_steps: String,
}

// ====================================================================
// LÓGICA DE EVALUACIÓN (Las 7 Reglas de Consultoría)
// ====================================================================

/// Ejecuta la evaluación del proyecto RWA en base a las 7 reglas
pub async fn evaluate_project_logic(submission: ProjectSubmission) -> EvaluationResult {
    let mut details: Vec<RuleDetail> = Vec::new();
    let mut total_score: f64 = 0.0;
    
    // --- Regla 1: Viabilidad en Solana (Activo) ---
    // Ponderación de 0.2 para activos de alto impacto como Certificaciones
    let (r1_compliance, r1_rationale, r1_score) = match submission.asset_type {
        AssetType::Certification => (true, "El activo de Certificación (anti-fraude) se alinea perfectamente con la visión de Solana y el mercado LATAM.", 0.2),
        _ => (false, "El tipo de activo no se alinea con las prioridades de alto impacto para la red.", 0.0),
    };
    details.push(RuleDetail {
        rule_name: "R1: Viabilidad en Solana (Activo)".to_string(),
        compliance: r1_compliance,
        rationale: r1_rationale.to_string(),
        score_impact: r1_score,
    });
    total_score += r1_score;

    // --- Regla 2: Reducción de Fraude (Impacto de Negocio) ---
    // Ponderación de 0.3, es la más importante.
    let r2_compliance = submission.description.to_lowercase().contains("fraude") || submission.description.to_lowercase().contains("trazabilidad");
    let (r2_rationale, r2_score) = if r2_compliance {
        ("El proyecto aborda directamente la problemática de fraude en documentos/activos.", 0.3)
    } else {
        ("El impacto directo en la reducción de fraude no es claro en la descripción.", 0.0)
    };
    details.push(RuleDetail {
        rule_name: "R2: Reducción de Fraude (White Paper)".to_string(),
        compliance: r2_compliance,
        rationale: r2_rationale.to_string(),
        score_impact: r2_score,
    });
    total_score += r2_score;

    // --- Regla 3: Monto Proporcional a Etapa ---
    // Ponderación de 0.1
    let (r3_compliance, r3_rationale, r3_score) = match submission.maturity_stage {
        MaturityStage::Idea if submission.requested_amount <= 50000.0 => (true, "Monto adecuado para la etapa 'Idea'.", 0.1),
        MaturityStage::MVP if submission.requested_amount <= 500000.0 => (true, "Monto adecuado para la etapa 'MVP'.", 0.1),
        MaturityStage::Scale if submission.requested_amount <= 2000000.0 => (true, "Monto adecuado para la etapa 'Escalamiento'.", 0.1),
        _ => (false, "El monto solicitado excede el límite máximo recomendado para la etapa de madurez del proyecto.", 0.0),
    };
    details.push(RuleDetail {
        rule_name: "R3: Monto Proporcional a Etapa".to_string(),
        compliance: r3_compliance,
        rationale: r3_rationale.to_string(),
        score_impact: r3_score,
    });
    total_score += r3_score;
    
    // NOTA: Se pueden añadir las reglas R4 a R7 para completar el modelo, pero con estas 3 es suficiente para la prueba MVP

    // --- Determinación del Estado Final ---
    let (status, next_steps) = if total_score >= 0.5 {
        ("Approved".to_string(), "El proyecto cumple con los requisitos iniciales. Procede a la fase de due diligence y estructuración del Smart Contract Anchor.".to_string())
    } else if total_score >= 0.2 {
        ("Conditionally Approved".to_string(), "El proyecto tiene potencial pero requiere revisar el impacto en la reducción de fraude o ajustar el monto solicitado.".to_string())
    } else {
        ("Rejected".to_string(), "El proyecto no cumple con los criterios fundamentales de viabilidad RWA en Solana.".to_string())
    };

    EvaluationResult {
        project_name: submission.project_name,
        status,
        total_score,
        details,
        next_steps,
    }
}
