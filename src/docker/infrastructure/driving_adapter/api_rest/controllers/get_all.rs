use actix_web::{get, HttpResponse, Responder};
use crate::docker::application::use_cases::get_all::GetAllUseCase;
use crate::docker::infrastructure::implementation::index::ImplementationRepository;

#[get("/get-all")]
async fn get() -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = GetAllUseCase::new(repo);
    
    let result = use_case.execute().await;

    HttpResponse::Ok().body(result)
}