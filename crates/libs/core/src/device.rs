use gnify::{vo::ID, Model};
use serde::{Deserialize, Serialize};

use crate::user::User;

mod bmc;
mod view;
mod vo;

pub use vo::*;
pub use bmc::*;
pub use view::*;


pub struct Device {
    pub(crate) name: DeviceName,
    pub(crate) session: Option<Session>,
    pub(crate) status: DeviceStatus,
}

impl Model for Device {
    type ID = DeviceToken;

    const NAME: &'static str = "Device";
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub token: SessionToken,
    pub user_id: ID<User>,
    pub expiration: ExpirationTimestamp,
}
