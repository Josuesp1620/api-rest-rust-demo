use crate::odoo::domain::repositories::index::Repository;
use std::error::Error;

pub struct StartUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> StartUseCase<R> {
    pub fn new(repository: R) -> Self {
        StartUseCase { repository }
    }

    pub async fn execute(&self, id: String) -> Result<(), Box<dyn Error>> {  
        self.repository.start(id).await?;
        Ok(())
    }
}