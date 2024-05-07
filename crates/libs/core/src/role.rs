use std::collections::HashSet;

use gnify::{
    error::InvalidValue,
    model::Record,
    vo::{Version, ID},
    Model,
};
use ulid::Ulid;

use crate::Privilege;

mod vo;
mod bmc;
mod view;

pub use vo::*;
pub use view::*;
pub use bmc::*;

#[derive(Debug, Model)]
pub struct Role {
    pub(crate) name: RoleName,
    pub(crate) level: RoleLevel,
    pub(crate) privileges: HashSet<Privilege>,
}

impl Role {
    pub fn new(
        id: Ulid,
        name: &str,
        level: &str,
        privileges: HashSet<&str>,
        author: Ulid,
    ) -> Result<Record<Self>, gnify::Error> {
        let id = ID::new(id);
        let state = Self {
            name: name.parse()?,
            level: level.parse()?,
            privileges: privileges
                .iter()
                .map(|name| name.parse())
                .collect::<Result<_, InvalidValue>>()?,
        };
        let version = Version::now(author);
        Ok(Record::new(id, state, version))
    }
}
