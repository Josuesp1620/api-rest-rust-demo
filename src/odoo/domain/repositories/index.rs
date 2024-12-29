use std::error::Error;

use crate::odoo::domain::{entities::index::ProjectEntity, mappers::index::ProjectResponse};

pub trait Repository {
    async fn create(&self, _project: ProjectEntity) -> Result<Option<ProjectResponse>, Box<dyn Error>>;
    async fn find_one(&self, _id:  &str) -> Result<Option<ProjectResponse>, Box<dyn Error>>;
    async fn start(&self, _id: String) -> Result<(), Box<dyn Error>>;
    async fn stop(&self, _id: String) -> Result<(), Box<dyn Error>>;
    async fn backup(&self, _id: String) -> Result<(), Box<dyn Error>>;
}
