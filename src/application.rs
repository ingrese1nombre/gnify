use std::{collections::HashSet, sync::Arc};

use gnify::source::{PgSource, Source};
use gnify_core::{
    role::{GetRole, Role, WriteRole},
    user::{GetUser, User, WriteUser},
};
use phf::{phf_map, Map};
use ulid::Ulid;

pub static PRIVILEGES: Map<&'static str, &'static [&'static str]> = phf_map! {
    "MANAGE USERS" => &[
        "REGISTER USER",
        "GET USER DETAILS"
    ],
    "MANAGE ROLES" => &[
        "REGISTER ROLES",
        "GET USER DETAILS"
    ]
};

pub struct AuthProfile {
    pub id: Ulid,
    pub privileges: HashSet<String>,
    pub level: u8,
}

#[derive(Clone)]
pub struct AppState {
    pub source: Arc<PgSource>,
}

impl AppState {
    pub async fn init(url: &'static str) -> gnify::error::Result<AppState> {
        let source = PgSource::new(url).await?;
        let role = source.read(GetRole::by_name("DEVELOPER")).await?;
        let role_id = if let Some(id) = role.map(|role| role.id()) {
            id.value()
        } else {
            let id = Ulid::new();
            let role = Role::new(id, "DEVELOPER", "Developer", HashSet::new(), Ulid::nil())?;
            source.write(WriteRole { record: role }).await?;
            id
        };
        let user = source.read(GetUser::by_username("developer")).await?;

        if user.is_none() {
            let user = User::new(
                Ulid::new(),
                "developer",
                "1234",
                None,
                Some(role_id),
                Ulid::nil(),
            )?;
            source.write(WriteUser { record: user }).await?;
        }
        Ok(Self {
            source: Arc::new(source),
        })
    }
}
