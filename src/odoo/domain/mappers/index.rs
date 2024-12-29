use serde::{Deserialize, Serialize};
use crate::odoo::infrastructure::driven_adapter::mongodb::index::Project;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponse {
    pub _id: String,
    pub name: String,
    pub version: String,
    pub odoo_type: String,
    pub path: String,
    pub port_web: u16,    
    pub port_db: u16,    
    pub master_password_odoo: String,    
}

impl TryFrom<Project> for ProjectResponse {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: Project) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: item._id.to_hex(),
            name: item.name,
            version: item.version,
            odoo_type: item.odoo_type,
            path: item.path,
            port_web: item.port_web,
            port_db: item.port_db,
            master_password_odoo: item.master_password_odoo,
        })
    }
}