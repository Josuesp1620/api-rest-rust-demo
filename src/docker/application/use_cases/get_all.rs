use crate::docker::domain::repositories::index::Repository;
use std::error::Error;
pub struct GetAllUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> GetAllUseCase<R> {
    pub fn new(repository: R) -> Self {
        GetAllUseCase { repository }
    }

    pub async fn execute(&self) -> Result<Option<String>, Box<dyn Error>>{
        return self.repository.get_all().await;
    }
}