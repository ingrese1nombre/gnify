use std::time::Duration;

use api_key::types::{ApiKeyResults, Default, StringGenerator};
use gnify::text;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};

text! {
    DeviceName: r"^\p{L}[\p{L}\s]{30}\p{L}$"
}

text! {
    DeviceToken: r"^\w{64}$"
}

text! {
    SessionToken: r"^\w{64}$"
}

impl SessionToken {
    pub fn generate() -> Self {
        let options = StringGenerator {
            prefix: String::from("GNI"),
            length: 64,
            ..StringGenerator::default()
        };

        let token = api_key::string(options);
        match token {
            ApiKeyResults::String(token) => Self(token),
            ApiKeyResults::StringArray(ref tokens) => Self(tokens[0].clone()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DeviceStatus {
    Authorized = 1,
    Unauthorized = 0,
}

impl From<i16> for DeviceStatus {
    fn from(value: i16) -> Self {
        if value == 1 { DeviceStatus::Authorized } else { DeviceStatus::Unauthorized }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExpirationTimestamp(NaiveDateTime);

impl std::ops::Deref for ExpirationTimestamp {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ExpirationTimestamp {
    pub fn new(offset: impl Into<Duration>) -> ExpirationTimestamp {
        let value = Utc::now().naive_utc() + offset.into();
        ExpirationTimestamp(value)
    }
}


impl From<NaiveDateTime> for ExpirationTimestamp {
    fn from(value: NaiveDateTime) -> Self {
        Self(value)
    }
}

impl From<ExpirationTimestamp> for NaiveDateTime {
    fn from(value: ExpirationTimestamp) -> Self {
        value.0
    }
}
