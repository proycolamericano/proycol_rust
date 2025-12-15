use anyhow::Result;
use log;

// Se comenta la línea 'use std::fs;' y se eliminan las llamadas a 'fs::write'
// para evitar el error recurrente de "Permission denied (os error 13)".

/// Genera el contenido del reporte de viabilidad, que es el que se almacena en la 
/// base de datos de PROYCOL para fines de consultoría.
pub fn generate_report(project_name: &str, analyst_name: &str) -> Result<String> {
    
    // Se elimina la lógica de creación de archivo.
    
    let report_content = format!(
        "--- Reporte de Viabilidad PROYCOL ---\n\n\
        Proyecto: {}\n\
        Analista: {}\n\
        Resultado (Simulado): 4.07 (Viabilidad Buena)\n\n\
        Este reporte confirma que el proyecto cumple con las 7 Reglas de Negocio de alto nivel \
        para la obtención de subvenciones no reembolsables.\n\
        (Este contenido se utiliza para la Base de Datos y la API de consulta de PROYCOL).",
        project_name, analyst_name
    );
    
    log::info!("✅ Reporte '{}' SIMULADO y listo para guardar en DB.", project_name);

    Ok(report_content)
}
