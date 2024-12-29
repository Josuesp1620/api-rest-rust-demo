use std::env;
use log::info;
use mongodb::{bson::doc, Client};

pub struct Database {
    client: mongodb::Client,
    db_name: String,
}

impl Database {
    pub async fn init() -> Self {
        let db_user_name = env::var("DB_USER_NAME").unwrap_or_else(|_| "dooservice".into());
        let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "dooservice".into());
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "27017".into());

        let uri = format!(
            "mongodb://{}:{}@{}:{}",
            db_user_name, db_password, db_host, db_port
        );

        let client: Client = Client::with_uri_str(&uri).await.unwrap();

        // Obtenemos el nombre de la base de datos una vez y lo almacenamos
        let db_name: String = env::var("DB_NAME").unwrap_or_else(|_| "dooservice".into());

        Database { client, db_name }
    }

    pub async fn check_connection(&self) -> Result<(), String> {
        match self.client.database(&self.db_name).run_command(doc! { "ping": 1 }).await {
            Ok(_) => {
                info!("Conexión exitosa a la base de datos");
                Ok(())
            }
            Err(e) => {
                Err(format!("Error de conexión a la base de datos: {}", e))
            }
        }
    }

    pub fn get_database(&self) -> mongodb::Database {
        self.client.database(&self.db_name)
    }
}
