use std::error::Error;
use crate::odoo::domain::entities::index::ProjectEntity;

pub trait Repository {
    async fn create(&self, _project: ProjectEntity) -> Result<Option<String>, Box<dyn Error>>;
    async fn start(&self, _id: String) -> Result<(), Box<dyn Error>>;
    async fn stop(&self, _id: String) -> Result<(), Box<dyn Error>>;
    async fn backup(&self, _id: String) -> Result<(), Box<dyn Error>>;
}