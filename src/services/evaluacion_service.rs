use crate::utils::pdf_generator::{self, ProjectData, EvaluationDetail, Plan}; 
use rusqlite::Connection;
use std::io; 

// Función placeholder (simulando la lógica real) - Aquí es donde irá la lógica de las 7 reglas
fn calculate_viability(_conn: &Connection, _data: &ProjectData) -> Result<(f32, Vec<EvaluationDetail>), Box<dyn std::error::Error>> {
    // Aquí implementaremos la lógica real en el próximo paso
    let details = vec![
        EvaluationDetail { name: "Rentabilidad (UMG)".to_string(), score: 5.0, comment: "Capital de operación (\\$10000) supera el Umbral Mínimo de Ganancia.".to_string() },
        EvaluationDetail { name: "Capacidad de Mercado".to_string(), score: 3.0, comment: "Mercado saturado, requiere estrategia disruptiva.".to_string() },
    ];
    let score = details.iter().map(|d| d.score).sum::<f32>() / details.len() as f32;
    Ok((score, details))
}


// Función principal para iniciar la evaluación y reporte
pub fn run_evaluation(db_conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    
    // 1. Obtener datos (Placeholder/Simulación)
    let mut report_data = ProjectData {
        company_name: "PROYCOL S.A.".to_string(),
        analyst_name: "JHON FREDY GONZALEZ VIVEROS".to_string(),
        viability_score: 0.0,
        evaluation_details: Vec::new(),
        implementation_plan: vec![ 
            Plan { level: "Fin".to_string(), indicators: "Plataforma lanzada.".to_string(), means_of_verification: "Registro de dominio.".to_string(), assumptions: "Mercado estable.".to_string() },
            Plan { level: "Propósito".to_string(), indicators: "10 clientes en 3 meses.".to_string(), means_of_verification: "Reporte de ventas.".to_string(), assumptions: "Financiación ok.".to_string() },
        ],
    };

    // 2. Calcular viabilidad
    let (score, details) = calculate_viability(db_conn, &report_data)?;
    report_data.viability_score = score;
    report_data.evaluation_details = details;


    // 3. Generar PDF
    match pdf_generator::generate_pdf(&report_data, "PROYCOL_Reporte_Final.pdf") {
        Ok(_) => {
            println!("Reporte 'PROYCOL_Reporte_Final.pdf' generado exitosamente.");
            Ok(())
        },
        Err(e) => {
            eprintln!("Error al generar el PDF: {}", e);
            Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("Error al generar PDF: {}", e))))
        }
    }
}
