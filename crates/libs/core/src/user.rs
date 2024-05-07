use std::collections::HashSet;

use gnify::{
    model::Record,
    vo::{Version, ID},
    Model,
};
use ulid::Ulid;

use crate::{role::Role, Privilege};

mod bmc;
mod view;
mod vo;

pub use bmc::*;
pub use view::*;
pub use vo::*;

#[derive(Model)]
pub struct User {
    pub(crate) username: Username,
    pub(crate) password: Password,
    pub(crate) email: Option<Email>,
    pub(crate) role_id: Option<ID<Role>>,
    pub(crate) privileges: HashSet<Privilege>,
}

impl User {
    pub fn new(
        id: Ulid,
        username: &str,
        password: &str,
        email: Option<&str>,
        role_id: Option<Ulid>,
        author: Ulid,
    ) -> Result<Record<User>, gnify::Error> {
        let state = User {
            username: username.parse()?,
            password: Password::generate(password)?,
            email: email.map(str::parse).transpose()?,
            role_id: role_id.map(ID::new),
            privileges: HashSet::new(),
        };
        let version = Version::now(author);
        Ok(Record::new(ID::new(id), state, version))
    }
}
