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
    println!("📡 API consultada: Enviando datos a PROYCOL...");
    let current_status = Status {
        project: "PROYCOL RWA".to_string(),
        network: "Solana Devnet".to_string(),
        status: "Online".to_string(),
    };
    HttpResponse::Ok().json(current_status)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("--------------------------------------");
    println!("🚀 PROYCOL BACKEND INICIADO");
    println!("📍 Servidor escuchando en: http://127.0.0.1:8081");
    println!("--------------------------------------");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors) // Permite la comunicación segura con el navegador
            .service(get_status)
    })
    .bind(("127.0.0.1", 8081))? // Usamos 127.0.0.1 y puerto 8081 para evitar el error 13
    .run()
    .await
}
