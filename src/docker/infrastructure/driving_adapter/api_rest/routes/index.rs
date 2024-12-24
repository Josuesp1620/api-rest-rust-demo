use actix_web::web;
use crate::docker::infrastructure::driving_adapter::api_rest::controllers;

pub fn docker_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/docker")
        .service(controllers::get_all::get)
        .service(controllers::get_by_id::get)
    );
}
