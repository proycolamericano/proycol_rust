use rusqlite::{Connection, Result, params}; 
use std::error::Error;

pub fn initialize_data(conn: &Connection) -> Result<(), Box<dyn Error>> {
    
    // Check if data already exists to avoid duplication (simple check)
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM project_rules WHERE name = ?1", 
                                params!["UMG"], 
                                |row| row.get(0))?;
    
    if count == 0 {
        // UMG (Umbral Mínimo de Ganancia) - e.g., 0.05%
        conn.execute(
            "INSERT INTO project_rules (name, threshold, current_value) VALUES (?1, ?2, ?3)",
            params!["UMG", 0.05_f64, 0.0_f64], 
        )?;
        
        // Capital de Operación (CO) - Max $1000
        conn.execute(
            "INSERT INTO project_rules (name, threshold, current_value) VALUES (?1, ?2, ?3)",
            params!["Capital de Operacion Max", 1000.0_f64, 0.0_f64], 
        )?;
        
        // Slippage Estimado - e.g., 0.10%
        conn.execute(
            "INSERT INTO project_rules (name, threshold, current_value) VALUES (?1, ?2, ?3)",
            params!["Slippage Estimado", 0.10_f64, 0.0_f64], 
        )?;
        
        println!("Datos iniciales (Reglas financieras) insertados en SQLite.");
    } else {
        println!("Datos iniciales ya existen en la base de datos.");
    }

    Ok(())
}
