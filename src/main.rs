use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files as fs;
use serde::Serialize;

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    project: String,
    network: String,
}

async fn status_handler() -> impl Responder {
    let response = StatusResponse {
        status: "Online".to_string(),
        project: "PROYCOL RWA".to_string(),
        network: "Solana Devnet Ready".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Iniciando servidor PROYCOL...");
    println!("📍 Accede a: http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // Endpoint de la API
            .route("/api/status", web::get().to(status_handler))
            // Servidor de archivos estáticos
            .service(fs::Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
