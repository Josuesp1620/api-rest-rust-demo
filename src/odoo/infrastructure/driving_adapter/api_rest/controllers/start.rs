use actix_web::{post, web, HttpResponse, Responder};
use crate::odoo::application::use_cases::start::StartUseCase;
use crate::odoo::infrastructure::implementation::index::ImplementationRepository;
use log::{error, info};

#[post("/start/{id}")]
async fn post(path: web::Path<(String,)>) -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = StartUseCase::new(repo);
    
    match use_case.execute(path.into_inner().0).await {
        Ok(()) => {
            info!(target: "[odoo -> controllers -> start]", "Acción completada exitosamente");
            HttpResponse::Ok().body("Acción completada exitosamente")
        },
        Err(e) => {
            error!(target: "[odoo -> controllers -> start]", "Error al realizar la acción: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}