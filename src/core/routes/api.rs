use actix_web::web;
use crate::docker::infrastructure::driving_adapter::api_rest::routes;

pub fn scope_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
        .configure(routes::index::docker_routes)
    );
}