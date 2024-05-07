use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::types::Uuid;
use ulid::Ulid;

use crate::error::InvalidValue;

pub trait Identifiable {
    type ID: Serialize
        + DeserializeOwned
        + Clone
        + FromStr
        + Display
        + Debug
        + Hash
        + PartialEq
        + Eq;
    const NAME: &'static str;
}

pub struct ID<T: Identifiable>(T::ID);

impl<T: Identifiable> Serialize for ID<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Identifiable> Deserialize<'de> for ID<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <T::ID as Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

impl<T: Identifiable> Eq for ID<T> {}

impl<T: Identifiable> PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Identifiable> Debug for ID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ID").field(&self.0).finish()
    }
}

impl<T: Identifiable> ID<T> {
    pub fn new(value: T::ID) -> Self {
        Self(value)
    }
    pub fn value(&self) -> T::ID {
        self.0.clone()
    }
}

impl<T: Identifiable> std::ops::Deref for ID<T> {
    type Target = T::ID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for ID<T>
where
    T: Identifiable,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for ID<T>
where
    T: Identifiable,
    T::ID: Copy,
{
}

impl<T: Identifiable> FromStr for ID<T> {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .parse()
            .map_err(|_| InvalidValue::new(format!("{} ID", T::NAME)))?;
        Ok(ID(s))
    }
}

impl <T: Identifiable<ID = Ulid>> From<Uuid> for ID<T> {
    fn from(value: Uuid) -> Self {
        Self(value.into())
    }
}

impl <T: Identifiable<ID = Ulid>> From<ID<T>> for Uuid {
    fn from(value: ID<T>) -> Self {
        Self::from(value.0)
    }
}
