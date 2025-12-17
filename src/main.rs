use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    project: String,
    network: String,
    status: String,
}

#[get("/api/status")]
async fn get_status() -> impl Responder {
    let current_status = Status {
        project: "PROYCOL RWA".to_string(),
        network: "Solana Devnet".to_string(),
        status: "Online".to_string(),
    };
    println!("✅ Petición recibida: Enviando estado a la web");
    HttpResponse::Ok().json(current_status)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Servidor PROYCOL iniciado en puerto 8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors) // Permite que el botón del HTML se comunique con Rust
            .service(get_status)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
