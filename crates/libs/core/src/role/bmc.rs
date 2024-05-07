use gnify::{model::Record, source::BMC};
use ulid::Ulid;

use super::{view::DetailedRoleView, Role};

mod postgres;

#[derive(Default)]
pub struct GetRole {
    id: Option<Ulid>,
    name: Option<String>,
}

impl GetRole {
    pub fn by_name(value: &str) -> Self {
        Self {
            name: Some(value.to_string()),
            ..Default::default()
        }
    }
}

impl BMC for GetRole {
    type Output = Option<DetailedRoleView>;
}

pub struct WriteRole {
    pub record: Record<Role>,
}
