use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::error::InvalidValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    author: Ulid,
    timestamp: NaiveDateTime,
}

impl Version {
    pub fn new(author: Ulid, timestamp: NaiveDateTime) -> Result<Self, InvalidValue> {
        if timestamp > Utc::now().naive_utc() {
            Err(InvalidValue::new("Version"))
        } else {
            Ok(Self { author, timestamp })
        }
    }

    pub fn now(author: Ulid) -> Self {
        Self {
            author,
            timestamp: Utc::now().naive_utc(),
        }
    }

    pub fn author(&self) -> Ulid {
        self.author
    }

    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
}
