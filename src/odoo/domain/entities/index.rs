// use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectEntity {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub odoo_type: String,
    pub path: Option<String>,
    pub port_web: Option<String>,
    pub random_port_web: bool,
    pub port_db: Option<String>,
    pub random_port_db: bool,
    pub master_password_odoo: Option<String>,
    pub random_master_password_odoo: bool,
}
