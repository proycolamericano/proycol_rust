// src/api_service.rs

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use anyhow::Result;

// Handler de la ruta raíz (Solo para probar que el servidor está vivo)
async fn index() -> impl Responder {
    HttpResponse::Ok().body("PROYCOL API: Servidor en línea (Historial de DB deshabilitado temporalmente)")
}

// La función principal del servidor debe ser PÚBLICA (pub)
pub async fn run_server() -> Result<()> {
    
    println!("🌐 Servidor de API de PROYCOL iniciado en http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // Ruta de prueba
            .route("/", web::get().to(index))
            // La ruta para el historial de proyectos (temporalmente desactivada porque la DB está comentada en main.rs)
            // .route("/api/history", web::get().to(get_history))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

// NOTA: La función get_history y sus estructuras de DB están omitidas 
// temporalmente porque la conexión a la base de datos está comentada en main.rs 
// para evitar el error de permisos.
