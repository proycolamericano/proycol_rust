// Archivo: src/handlers/mod.rs
use actix_web::{web, HttpResponse, Responder};
use crate::core::evaluator::{ProjectSubmission, EvaluationResult, evaluate_project_logic};
use serde_json::json;
// Necesitamos importar Arc<Connection> si vamos a usar la DB en la siguiente fase
// use rusqlite::Connection;
// use std::sync::Arc;

// Handler para la ruta POST /evaluate
pub async fn evaluate_project(
    // Acceso a la base de datos (descomentar en el Hito M2)
    // db_pool: web::Data<Arc<Connection>>,
    submission: web::Json<ProjectSubmission>,
) -> impl Responder {

    // Ejecutar la lógica de evaluación (aplica las 7 reglas)
    let result: EvaluationResult = evaluate_project_logic(submission.into_inner()).await;

    // Responder al frontend con el resultado en formato JSON
    HttpResponse::Ok().json(json!({
        "project_name": result.project_name,
        "status": result.status,
        "total_score": result.total_score,
        "details": result.details,
        "next_steps": result.next_steps,
        "message": "Evaluación de Consultoría RWA completada exitosamente."
    }))
}
