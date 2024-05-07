use std::collections::HashSet;

use gnify::{model::Record, vo::{Version, ID}};
use serde::Serialize;

use crate::{role::{RoleLevel, RoleName, Role}, Privilege};

use super::{vo::{Email, Password, Username}, User};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DetailedUserView {
    pub(crate) id: ID<User>,
    pub(crate) username: Username,
    #[serde(skip)]
    pub(crate) password: Password,
    pub(crate) email: Option<Email>,
    pub(crate) role: Option<UserRole>,
    pub(crate) privileges: HashSet<Privilege>,
    pub(crate) version: Version,
    pub(crate) first_version: Version
}

impl DetailedUserView {
    pub fn as_record(self) -> Record<User> {
        let DetailedUserView { id, username, password, email, role, privileges, version, first_version: _ } = self;
        let state = User { username, password, email, role_id: role.map(|role| role.id), privileges };
        Record::new(id, state, version)
    }
    
    pub fn id(&self) -> ID<User> {
        self.id
    }
    
    pub fn username(&self) -> &Username {
        &self.username
    }
    
    pub fn password(&self) -> &Password {
        &self.password
    }
    
    pub fn email(&self) -> Option<&Email> {
        self.email.as_ref()
    }
    
    pub fn role(&self) -> Option<&UserRole> {
        self.role.as_ref()
    }
    
    pub fn privileges(&self) -> &HashSet<Privilege> {
        &self.privileges
    }
    
    pub fn version(&self) -> Version {
        self.version
    }
    
    pub fn first_version(&self) -> Version {
        self.first_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserRole {
    pub id: ID<Role>,
    pub name: RoleName, 
    pub level: RoleLevel,
    pub privileges: HashSet<Privilege>
}