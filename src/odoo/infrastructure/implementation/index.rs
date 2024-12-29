use crate::core::services::mongodb_connector::index::Database;
use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::domain::mappers::index::ProjectResponse;
use crate::odoo::domain::repositories::index::Repository;
use crate::odoo::infrastructure::driven_adapter::mongodb::index::Project;
use std::error::Error;
use std::str::FromStr;
use actix_web::web::Data;
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::Collection;

pub struct ImplementationRepository {
    db: Collection<Project>,
}

impl ImplementationRepository {
    pub fn new(db: Data<Database>) -> Self {
        ImplementationRepository { db: db.get_database().collection("project") }
    }
}

impl Repository for ImplementationRepository {
    async fn create(&self, _project: ProjectEntity) -> Result<Option<ProjectResponse>, Box<dyn Error>> {
        match self.db.insert_one(Project::try_from(_project)?).await
        {
            Ok(created) => {                
                self.find_one(&created.inserted_id.as_object_id().unwrap().to_hex()).await
            }
            Err(err) => {                
                Err(Box::new(err))
            }
        }
    }    
    
    async fn find_one(&self, _id: &str) -> Result<Option<ProjectResponse>, Box<dyn Error>> {
        let filter = doc! { "_id": ObjectId::from_str(_id).expect("Failed to parse booking_id") };

        match self.db.find_one(filter).await {
            Ok(Some(full_project)) => {
                Ok(Some(ProjectResponse::try_from(full_project)?))
            }
            Ok(None) => {
                Ok(None)
            }
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }   

    async fn start(&self, _id: String) -> Result<(), Box<dyn Error>> {
        let logger_info: String = format!("Obteniendo elemento con id: {}", _id);
        info!(target: "[odoo -> ImplementationRepository -> start]", "{}", logger_info);
        Ok(())
    }
    async fn stop(&self, _id: String) -> Result<(), Box<dyn Error>> {
        info!(target: "[odoo -> ImplementationRepository -> stop]", "Deteniendo Project");
        Ok(())
    }
    async fn backup(&self, _id: String) -> Result<(), Box<dyn Error>> {
        info!(target: "[odoo -> ImplementationRepository -> backup]", "Creando Backup de Project");
        Ok(())
    }
}
