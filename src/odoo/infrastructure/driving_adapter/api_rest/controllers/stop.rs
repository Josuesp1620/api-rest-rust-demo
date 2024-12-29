use actix_web::{post, web, HttpResponse, Responder};
use crate::{core::services::mongodb_connector::index::Database, odoo::application::use_cases::stop::StopUseCase};
use crate::odoo::infrastructure::implementation::index::ImplementationRepository;
use log::{error, info};

#[post("/stop/{id}")]
async fn post(db: web::Data<Database>, path: web::Path<(String,)>) -> impl Responder {
    let repo: ImplementationRepository = ImplementationRepository::new(db);
    let use_case = StopUseCase::new(repo);
    
    match use_case.execute(path.into_inner().0).await {
        Ok(()) => {
            info!(target: "[odoo -> controllers -> stop]", "Acción completada exitosamente");
            HttpResponse::Ok().body("Acción completada exitosamente")
        },
        Err(e) => {
            error!(target: "[odoo -> controllers -> stop]", "Error al realizar la acción: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}