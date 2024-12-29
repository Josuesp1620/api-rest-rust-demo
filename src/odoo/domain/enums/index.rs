use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OdooType {
    Community,
    Enterprise,
}

impl OdooType {
    pub fn as_str(&self) -> &str {
        match self {
            OdooType::Community => "Community",
            OdooType::Enterprise => "Enterprise",
        }
    }
}
