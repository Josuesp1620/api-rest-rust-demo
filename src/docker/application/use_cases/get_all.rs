use crate::docker::domain::repositories::index::Repository;

pub struct GetAllUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> GetAllUseCase<R> {
    pub fn new(repository: R) -> Self {
        GetAllUseCase { repository }
    }

    pub fn execute(&self) -> String{
        return self.repository.get_all();
    }
}