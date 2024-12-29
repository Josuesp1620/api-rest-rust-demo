use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OdooType {
    CC,
    EE,
}

impl OdooType {
    pub fn as_str(&self) -> &str {
        match self {
            OdooType::CC => "CC",
            OdooType::EE => "EE",
        }
    }
}
