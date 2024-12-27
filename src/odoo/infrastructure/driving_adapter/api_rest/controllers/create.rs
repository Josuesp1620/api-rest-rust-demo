use actix_web::{post, web, HttpResponse, Responder};
use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::application::use_cases::create::CreateUseCase;
use crate::odoo::infrastructure::implementation::index::ImplementationRepository;
use log::{error, info};

#[post("/create")]
async fn post(project: web::Json<ProjectEntity>) -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = CreateUseCase::new(repo);
    
    let project_entity = project.into_inner();
    
    match use_case.execute(project_entity).await {
        Ok(Some(result)) => {
            info!(target: "[odoo -> controllers -> create]", "Elementos obtenidos exitosamente");
            HttpResponse::Ok().body(result)
        },
        Ok(None) => {
            info!(target: "[odoo -> controllers -> create]", "No se encontraron elementos");
            HttpResponse::NotFound().body("No se encontraron elementos")
        },
        Err(e) => {
            error!(target: "[odoo -> controllers -> create]", "Error al obtener todos los elementos: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}