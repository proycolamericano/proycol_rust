// src/utils/db_manager.rs

use rusqlite::{Connection, Result, params};
use chrono::Local;
use log;
use serde::{Serialize, Deserialize};

// Suponiendo que tienes una función initialize_db existente.

// Estructura para mapear las filas de la DB (necesario para serializar a JSON)
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectResult {
    pub id: i32,
    pub project_name: String,
    pub analyst_name: String,
    pub final_score: f32,
    pub viability_grade: String,
    pub evaluation_date: String,
}

/// Guarda el resultado de un proyecto generado en la tabla de historial.
pub fn save_project_result(
    conn: &Connection, 
    project_name: &str, 
    analyst_name: &str, 
    final_score: f32, 
    viability: &str
) -> Result<()> {
    
    // 1. Asegurar que la tabla de historial existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project_history (
            id INTEGER PRIMARY KEY,
            project_name TEXT NOT NULL,
            analyst_name TEXT NOT NULL,
            final_score REAL NOT NULL,
            viability_grade TEXT NOT NULL,
            evaluation_date TEXT NOT NULL
        )", 
        [],
    )?;

    // 2. Insertar el nuevo resultado
    conn.execute(
        "INSERT INTO project_history (project_name, analyst_name, final_score, viability_grade, evaluation_date) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            project_name,
            analyst_name,
            final_score,
            viability,
            Local::now().to_string()
        ],
    )?;

    log::info!("✅ Resultado del proyecto '{}' guardado en el historial (Puntuación: {}).", project_name, final_score);
    
    Ok(())
}


/// Lee todos los proyectos del historial de la base de datos.
pub fn get_all_project_results(conn: &Connection) -> Result<Vec<ProjectResult>> {
    let mut stmt = conn.prepare("SELECT id, project_name, analyst_name, final_score, viability_grade, evaluation_date FROM project_history ORDER BY evaluation_date DESC")?;
    
    let project_iter = stmt.query_map([], |row| {
        Ok(ProjectResult {
            id: row.get(0)?,
            project_name: row.get(1)?,
            analyst_name: row.get(2)?,
            final_score: row.get(3)?,
            viability_grade: row.get(4)?,
            evaluation_date: row.get(5)?,
        })
    })?;

    let results: Result<Vec<ProjectResult>> = project_iter.collect();
    results
}
