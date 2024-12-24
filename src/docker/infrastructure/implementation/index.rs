use crate::docker::domain::repositories::index::Repository;
use bollard::Docker;

pub struct ImplementationRepository;

impl Repository for ImplementationRepository {
    async fn get_all(&self) -> String {    
        let docker = Docker::connect_with_socket_defaults().expect("No se pudo conectar a Docker");
        let version = docker.version().await.unwrap();
        match version.version {
            Some(version) => version,
            None => "Versión no disponible".to_string(),
        }
    }

    async fn get_by_id(&self, id: String) -> String {
        let message = format!("Obteniendo elemento con id: {}", id);
        message
    }
}
