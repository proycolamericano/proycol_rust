// src/db/mod.rs

use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::Mutex; // Necesario para el struct AppState

// Estructura original para las reglas de arbitraje
pub struct Rule {
    pub id: i32,
    pub formula: String,
    pub enabled: i32,
}

// Define la estructura para el estado de la aplicación (usada en main.rs)
pub struct AppState {
    pub db_pool: Mutex<Connection>,
}

pub fn init_db(db_path: &str) -> rusqlite::Result<Connection> {
    let path = Path::new(db_path);
    let parent = path.parent().unwrap();

    // 1. Verificar/Crear directorio de datos
    if !parent.exists() {
        fs::create_dir_all(parent).expect("Error al crear el directorio de datos");
        println!("✅ Directorio de datos creado/verificado: {}", parent.display());
    } else {
        println!("✅ Directorio de datos verificado: {}", parent.display());
    }

    // 2. Conectar/Crear DB
    let conn = Connection::open(db_path)?;

    // 3. Crear tabla de Reglas de Arbitraje
    conn.execute(
        "CREATE TABLE IF NOT EXISTS arbitrage_rules (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            formula TEXT NOT NULL,
            enabled INTEGER NOT NULL DEFAULT 1
        )",
        (),
    )?;

    // 4. Crear tabla de Evaluación de Proyectos (NUEVA TABLA PARA PROYCOL CORE)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS evaluations (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            score       REAL NOT NULL,
            is_viable   INTEGER NOT NULL,
            created_at  TEXT NOT NULL
        )",
        (),
    )?;

    // 5. Insertar las 7 reglas iniciales (si la tabla está vacía)
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM arbitrage_rules",
        [],
        |row| row.get(0),
    )?;

    if count == 0 {
        // Estas 7 reglas son placeholders para el esquema.
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (1, 'Market Size', 'market_size > 10M')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (2, 'Investment Ratio', 'capital / market < 0.5')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (3, 'Expected ROI', 'roi > 30%')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (4, 'Regulatory Risk', 'risk_score < 7')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (5, 'Team Experience', 'years > 5')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (6, 'DB Capability', 'has_db = TRUE')", ())?;
        conn.execute("INSERT INTO arbitrage_rules (id, name, formula) VALUES (7, 'Maturity Stage', 'stage != Idea')", ())?;

        println!("✅ 7 Reglas de Arbitraje iniciales insertadas.");
    } else {
        println!("✅ 7 Reglas de Arbitraje verificadas.");
    }

    Ok(conn)
}
