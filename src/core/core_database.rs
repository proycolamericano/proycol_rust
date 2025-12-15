use rusqlite::{Connection, Result}; 

pub struct DbManager; // Aseguramos que el nombre sea DbManager

impl DbManager {
    /// Inicializa la conexión a la base de datos (crea el archivo si no existe)
    pub fn new() -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open("proycol.db")?;
        Ok(conn)
    }

    /// Crea las tablas necesarias para guardar la información del proyecto y las reglas.
    pub fn initialize_db(conn: &Connection) -> Result<()> {
        // Tabla para los parámetros de las 7 Reglas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_rules (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                threshold REAL NOT NULL,
                current_value REAL
            )",
            [],
        )?;
        
        // Tabla para los datos generales del proyecto
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project_data (
                id INTEGER PRIMARY KEY,
                company_name TEXT NOT NULL,
                analyst_name TEXT NOT NULL,
                viability_score REAL
            )",
            [],
        )?;
        
        Ok(())
    }
}
