pub trait Repository {
    fn get_all(&self) -> String;
    fn get_by_id(&self, id: String) -> String;
}