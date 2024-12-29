use actix_web::{post, web, HttpResponse, Responder};
use crate::{core::services::mongodb_connector::index::Database, odoo::domain::entities::index::ProjectEntity};
use crate::odoo::application::use_cases::create::CreateUseCase;
use crate::odoo::infrastructure::implementation::index::ImplementationRepository;
use log::error;

#[post("/create")]
async fn post(db: web::Data<Database>, project: web::Json<ProjectEntity>) -> impl Responder {
    let repo: ImplementationRepository = ImplementationRepository::new(db);
    let use_case = CreateUseCase::new(repo);
    
    let project_entity = project.into_inner();
    
    match use_case.execute(project_entity).await {
        Ok(Some(result)) => {
            HttpResponse::Ok().json(result)
        },
        Ok(None) => {
            HttpResponse::NotFound().body("No se encontraron elementos")
        },
        Err(e) => {
            error!("Error al obtener todos los elementos: {}", e);
            HttpResponse::InternalServerError().body("Error interno del servidor")
        }
    }
}