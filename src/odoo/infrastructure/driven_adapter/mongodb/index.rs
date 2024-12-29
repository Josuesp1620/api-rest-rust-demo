use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::odoo::domain::entities::index::ProjectEntity;

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub _id: ObjectId,
    pub name: String,
    pub version: String,
    pub odoo_type: String,
    pub path: String,
    pub port_web: u16,    
    pub port_db: u16,    
    pub master_password_odoo: String,    
}

impl TryFrom<ProjectEntity> for Project {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: ProjectEntity) -> Result<Self, Self::Error> {

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            version: item.version,
            odoo_type: item.odoo_type,
            path: match item.path {
                Some(val) => val,
                None => "default".to_string(),
            },
            port_web: match item.port_web {
                Some(val) => val,
                None => 8069,
            },
            port_db: match item.port_db {
                Some(val) => val,
                None => 5432,
            },
            master_password_odoo: match item.master_password_odoo {
                Some(val) => val,
                None => "default_password".to_string(),
            },
        })
    }
}