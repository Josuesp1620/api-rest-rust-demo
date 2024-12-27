use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::domain::repositories::index::Repository;
use std::error::Error;

pub struct CreateUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> CreateUseCase<R> {
    pub fn new(repository: R) -> Self {
        CreateUseCase { repository }
    }

    pub async fn execute(&self, project: ProjectEntity) -> Result<Option<String>, Box<dyn Error>>{  
        return self.repository.create(project).await;
    }
}