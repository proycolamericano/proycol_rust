use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;
use std::io::Result;
use rusqlite::Connection;
use std::sync::Arc;

// Modulos requeridos
mod handlers;
mod core;
mod db; 

// Base de datos (necesario para la inicialización)
use crate::core::core_database::DbManager;


#[actix_web::main]
async fn main() -> Result<()> {
    
    // 1. Inicializar y verificar el esquema de la base de datos (Ejecución fuera del servidor Actix)
    // NOTA: La conexión no se comparte con los threads del servidor para evitar el error E0277.
    match DbManager::new() {
        Ok(conn) => {
            println!("✅ Conexión a SQLite establecida.");
            match DbManager::initialize_db(&conn) {
                Ok(_) => println!("✅ Esquema de base de datos verificado."),
                Err(e) => eprintln!("❌ Error al inicializar el esquema de la DB: {}", e),
            }
        }
        Err(e) => {
            eprintln!("❌ Error al conectar a la DB: {}", e);
            // Esto es un error fatal para el proyecto Proycol.
            panic!("No se pudo iniciar la base de datos.");
        }
    };
    
    // 2. Configurar el servidor Actix-Web
    println!("🌐 Inicializando Servidor de API de PROYCOL...");

    HttpServer::new(move || {
        // Configuración de CORS: Permite peticiones desde el frontend HTML
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            
            // ----------------------------------------------------
            // REGISTRO DE ENDPOINTS
            // ----------------------------------------------------
            
            // **ENDPOINT CRÍTICO: EVALUACIÓN DE PROYECTOS (Hito M1)**
            .route("/evaluate", web::post().to(handlers::evaluate_project)) 

            // ENDPOINT DE SALUD (Health Check)
            .route("/ping", web::get().to(|| async { "API PROYCOL RWA está activa" }))
            
    })
    // El servidor escucha en la dirección y puerto estándar (para WSL)
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
