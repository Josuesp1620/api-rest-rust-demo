pub trait Repository {
    async fn get_all(&self) -> String;
    async fn get_by_id(&self, id: String) -> String;
}