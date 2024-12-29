use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::core::routes::api;
use crate::core::services::mongodb_connector::index::Database;

pub struct Server {
    port: u16,
    host: &'static str,
}

impl Server {
    pub fn new(host: &'static str, port: u16) -> Self {
        Server {
            port,
            host,
        }
    }

    #[actix_web::main]
    pub async fn listen(&self) -> std::io::Result<()> {
        let host = self.host;
        let port = self.port;

        env_logger::init_from_env(Env::default().default_filter_or("info"));

        let db = Database::init().await;
        match db.check_connection().await {
            Ok(_) => {
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
            Err(e) => {
                Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused, 
                    format!("{}", e)
                ))
            }
        }
    }
}
