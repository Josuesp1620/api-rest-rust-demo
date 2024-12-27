use crate::odoo::domain::repositories::index::Repository;
use std::error::Error;

pub struct StopUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> StopUseCase<R> {
    pub fn new(repository: R) -> Self {
        StopUseCase { repository }
    }

    pub async fn execute(&self, id: String) -> Result<(), Box<dyn Error>>{
        self.repository.stop(id).await?;
        Ok(())
    }
}