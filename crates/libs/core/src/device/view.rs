use gnify::vo::Version;
use serde::{Deserialize, Serialize};

use super::{DeviceName, DeviceStatus, DeviceToken, Session};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceView {
    pub(crate) token: DeviceToken,
    pub(crate) version: Version,
    pub(crate) first_version: Version,
    pub(crate) name: DeviceName,
    pub(crate) session: Option<Session>,
    pub(crate) status: DeviceStatus
}