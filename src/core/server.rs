use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::core::routes::api;
use crate::core::services::mongodb_connector::index::Database;

pub struct Server {
    _port: u16,
    _host: &'static str,
}

impl Server {
    pub fn new(host: &'static str, port: u16) -> Self {
        Server {
            _port: port,
            _host: host,
        }
    }

    #[actix_web::main]
    pub async fn listen(&self) -> std::io::Result<()> {
        let host = self._host;
        let port = self._port;

        env_logger::init_from_env(Env::default().default_filter_or("info"));

        let db = Database::init().await;
        let db_data = web::Data::new(db);

        HttpServer::new(move || {
            App::new()
                .app_data(db_data.clone())
                .configure(api::scope_routes)
                .wrap(Logger::default())
        })
        .bind((host, port))?
        .run()
        .await
    }
}