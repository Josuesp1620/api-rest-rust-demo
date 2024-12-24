use crate::docker::domain::repositories::index::Repository;

pub struct ImplementationRepository;

impl Repository for ImplementationRepository {
    fn get_all(&self) -> String {
        let message = "Obteniendo todos los elementos.".to_string();
        message
    }

    fn get_by_id(&self, id: String) -> String {
        let message = format!("Obteniendo elemento con id: {}", id);
        message
    }
}
