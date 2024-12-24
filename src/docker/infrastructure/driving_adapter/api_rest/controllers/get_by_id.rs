use actix_web::{get, HttpResponse, Responder};
use crate::docker::application::use_cases::get_by_id::GetByIdUseCase;
use crate::docker::infrastructure::implementation::index::ImplementationRepository;

#[get("/get-by-id")]
async fn get() -> impl Responder {
    let repo = ImplementationRepository;
    let use_case = GetByIdUseCase::new(repo);
    
    // Ejecutamos el caso de uso
    let result = use_case.execute("demo".to_string()).await;

    // Retornamos una respuesta HTTP válida
    HttpResponse::Ok().body(result)
}