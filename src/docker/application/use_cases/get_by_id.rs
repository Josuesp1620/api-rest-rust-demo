use crate::docker::domain::repositories::index::Repository;

pub struct GetByIdUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> GetByIdUseCase<R> {
    pub fn new(repository: R) -> Self {
        GetByIdUseCase { repository }
    }

    pub async fn execute(&self, id: String) -> String{
        return self.repository.get_by_id(id).await;
    }
}