use std::collections::HashSet;

use gnify::{model::Record, vo::{Version, ID}};
use serde::Serialize;

use crate::Privilege;

use super::{Role, RoleLevel, RoleName};

#[derive(Debug, Serialize)]
pub struct DetailedRoleView {
    pub(crate) id: ID<Role>,
    pub(crate) version: Version,
    pub(crate) first_version: Version,
    pub(crate) name: RoleName,
    pub(crate) level: RoleLevel,
    pub(crate) privileges: HashSet<Privilege>,
}

impl DetailedRoleView {
    pub fn as_record(self) -> Record<Role> {
        let DetailedRoleView { id, version, first_version: _, name, level, privileges } = self;
        let state = Role {
            name,
            level,
            privileges
        };
        Record::new(id, state, version)
    }
    pub fn id(&self) -> ID<Role> {
        self.id
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn first_version(&self) -> Version {
        self.first_version
    }

    pub fn name(&self) -> &RoleName {
        &self.name
    }

    pub fn level(&self) -> RoleLevel {
        self.level
    }

    pub fn privileges(&self) -> &HashSet<Privilege> {
        &self.privileges
    }
}