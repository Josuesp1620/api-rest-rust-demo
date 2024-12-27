use crate::odoo::domain::repositories::index::Repository;
use std::error::Error;

pub struct BackupUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> BackupUseCase<R> {
    pub fn new(repository: R) -> Self {
        BackupUseCase { repository }
    }

    pub async fn execute(&self, id: String) -> Result<(), Box<dyn Error>>{
        self.repository.backup(id).await?;
        Ok(())
    }
}