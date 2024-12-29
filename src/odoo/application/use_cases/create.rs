use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::domain::mappers::index::ProjectResponse;
use crate::odoo::domain::repositories::index::Repository;
use crate::odoo::infrastructure::driven_adapter::generate_port::{find_available_port, generate_random_password};
use std::error::Error;

pub struct CreateUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> CreateUseCase<R> {
    pub fn new(repository: R) -> Self {
        CreateUseCase { repository }
    }

    pub async fn execute(&self, mut project: ProjectEntity) -> Result<Option<ProjectResponse>, Box<dyn Error>> {
        self.repository.create(project).await
    }    
}