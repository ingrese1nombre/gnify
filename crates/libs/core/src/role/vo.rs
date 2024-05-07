use std::str::FromStr;

use gnify::{error::InvalidValue, text};
use serde::{Deserialize, Serialize};

text! {
    RoleName: r"^(\p{L}+\s)*\p{L}+$"
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type, Clone, Copy, Default,
)]
#[repr(u8)]
pub enum RoleLevel {
    Developer = 4,
    Administrator = 3,
    Manager = 2,
    Operator = 1,
    #[default]
    Guest = 0,
}

impl RoleLevel {
    pub fn from(value: impl TryInto<u8>) -> Self {
        let value = value.try_into()
            .unwrap_or(0);

        match value {
            4 => Self::Developer,
            3 => Self::Administrator,
            2 => Self::Manager,
            1 => Self::Operator,
            _ => Self::Guest
        }
    }
}

impl FromStr for RoleLevel {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "developer" => Ok(RoleLevel::Developer),
            "administrator" => Ok(RoleLevel::Administrator),
            "manager" => Ok(RoleLevel::Manager),
            "operator" => Ok(RoleLevel::Operator),
            "guest" => Ok(RoleLevel::Guest),
            _ => Err(InvalidValue::new("RoleLevel")),
        }
    }
}
