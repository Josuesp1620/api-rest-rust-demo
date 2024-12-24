use crate::docker::domain::repositories::index::Repository;
use bollard::Docker;
use std::error::Error;

pub struct ImplementationRepository;

impl Repository for ImplementationRepository {
    async fn get_all(&self) -> Result<Option<String>, Box<dyn Error>> {
        let docker = Docker::connect_with_socket_defaults()?;
        let version = docker.version().await?;
        Ok(version.version)
    }

    async fn get_by_id(&self, id: String) -> String {
        format!("Obteniendo elemento con id: {}", id)
    }
}
