use actix_web::{post, web, HttpResponse, Responder};
use crate::odoo::application::use_cases::backup::BackupUseCase;
use crate::odoo::infrastructure::implementation::index::ImplementationRepository;
use log::{error, info};

#[post("/backup/{id}")]
async fn post(path: web::Path<(String,)>) -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = BackupUseCase::new(repo);
    
    match use_case.execute(path.into_inner().0).await {
        Ok(()) => {
            info!(target: "[odoo -> controllers -> backup]", "Acción completada exitosamente");
            HttpResponse::Ok().body("Acción completada exitosamente")
        },
        Err(e) => {
            error!(target: "[odoo -> controllers -> backup]", "Error al realizar la acción: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}