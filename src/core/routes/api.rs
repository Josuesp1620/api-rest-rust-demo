use actix_web::web;
use crate::docker::infrastructure::driving_adapter::api_rest::routes::index::docker_routes;
use crate::odoo::infrastructure::driving_adapter::api_rest::routes::index::odoo_routes;

pub fn scope_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
        .configure(docker_routes)
        .configure(odoo_routes)
    );
}