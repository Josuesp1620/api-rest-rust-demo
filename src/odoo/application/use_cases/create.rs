use crate::odoo::domain::entities::index::ProjectEntity;
use crate::odoo::domain::mappers::index::ProjectResponse;
use crate::odoo::domain::repositories::index::Repository;
use crate::odoo::infrastructure::driven_adapter::generate_port::{find_available_port, generate_random_password};
use std::error::Error;

pub struct CreateUseCase<R: Repository> { 
    repository: R,
}

impl<R: Repository> CreateUseCase<R> {
    pub fn new(repository: R) -> Self {
        CreateUseCase { repository }
    }

    pub async fn execute(&self, mut project: ProjectEntity) -> Result<Option<ProjectResponse>, Box<dyn Error>> {
        if project.random_master_password_odoo && project.master_password_odoo.is_none() {
            match generate_random_password(24) {
                Some(password) => {
                    project.master_password_odoo = Some(password);
                }
                None => println!("Error al generar la contraseña."),
            }
        }
        if project.random_port_db && project.port_db.is_none() {
            let odoo_postgres_port = find_available_port(5000, 5999);
            project.port_db = Some(odoo_postgres_port);
        }
        if project.random_port_web && project.port_web.is_none() {
            let odoo_web_port = find_available_port(8000, 8999);
            project.port_web = Some(odoo_web_port);            
        }
        self.repository.create(project).await
    }    
}