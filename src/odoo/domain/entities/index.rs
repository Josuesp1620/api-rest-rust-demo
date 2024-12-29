use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectEntity {
    pub _id: Option<String>,
    pub name: String,
    pub version: String,
    pub odoo_type: String,
    pub path: Option<String>,
    pub port_web: Option<u16>,
    pub random_port_web: bool,
    pub port_db: Option<u16>,
    pub random_port_db: bool,
    pub master_password_odoo: Option<String>,
    pub random_master_password_odoo: bool,
}
