use std::env;
use mongodb::{Client, Collection};
use crate::odoo::infrastructure::driven_adapter::mongodb::index::Project;

pub struct Database {
    project: Collection<Project>,
}

impl Database {
    pub async fn init() -> Self {

        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "dooservice".into());
        let db_user_name = env::var("DB_USER_NAME").unwrap_or_else(|_| "dooservice".into());
        let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "dooservice".into());
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "27017".into());
    
        let uri = format!(
            "mongodb://{}:{}@{}:{}",
            db_user_name, db_password, db_host, db_port
        );        

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(&db_name);        
        let project: Collection<Project> = db.collection("project");

        Database {
            project,
        }
    }

    // Método para acceder a la colección de 'project'
    pub fn get_project_collection(&self) -> &Collection<Project> {
        &self.project
    }
}