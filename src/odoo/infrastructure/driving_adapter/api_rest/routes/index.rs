use actix_web::web;
use crate::odoo::infrastructure::driving_adapter::api_rest::controllers;

pub fn odoo_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/odoo")
        .service(controllers::create::post)
        .service(controllers::start::post)
        .service(controllers::stop::post)
        .service(controllers::backup::post)
    );
}
