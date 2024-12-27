use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::domain::repositories::index::Repository;
use std::error::Error;
use log::info;

pub struct ImplementationRepository;

impl Repository for ImplementationRepository {
    // async fn get_all(&self) -> Result<Option<String>, Box<dyn Error>> {
    //     let docker = Docker::connect_with_socket_defaults()?;
    //     let version = docker.version().await?;
    //     Ok(version.version)
    // }

    async fn create(&self, _project: ProjectEntity) -> Result<Option<String>, Box<dyn Error>> {
        let logger_info = format!("Creando Proyecto: {:?}", _project);
        info!(target: "[odoo -> ImplementationRepository -> create]", "{}", logger_info);
        let id_return = "123456789".to_string();
        Ok(Some(id_return))
    }
    
    async fn start(&self, _id: String) -> Result<(), Box<dyn Error>> {
        let logger_info = format!("Obteniendo elemento con id: {}", _id);
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
