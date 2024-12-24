use actix_web::{get, HttpResponse, Responder};
use crate::docker::application::use_cases::get_all::GetAllUseCase;
use crate::docker::infrastructure::implementation::index::ImplementationRepository;
use log::{error, info};

#[get("/get-all")]
async fn get() -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = GetAllUseCase::new(repo);
    
    match use_case.execute().await {
        Ok(Some(result)) => {
            // info!(target: "[docker -> controllers -> get_all]", "Elementos obtenidos exitosamente");
            HttpResponse::Ok().body(result)
        },
        Ok(None) => {
            // info!(target: "[docker -> controllers -> get_all]", "No se encontraron elementos");
            HttpResponse::NotFound().body("No se encontraron elementos")
        },
        Err(e) => {
            error!(target: "[docker -> controllers -> get_all]", "Error al obtener todos los elementos: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}