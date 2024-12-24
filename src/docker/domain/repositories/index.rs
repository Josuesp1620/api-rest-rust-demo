use std::error::Error;

pub trait Repository {
    async fn get_all(&self) -> Result<Option<String>, Box<dyn Error>>;
    async fn get_by_id(&self, id: String) -> String;
}