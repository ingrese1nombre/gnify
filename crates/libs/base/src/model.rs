use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

use serde::{de::DeserializeOwned, Serialize};
use ulid::Ulid;

use crate::vo::{Identifiable, Version, ID};

pub trait Model {
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

impl<T: Model> Identifiable for T {
    type ID = T::ID;
    const NAME: &'static str = T::NAME;
}

pub trait RecordUpdate: Clone + PartialEq + Eq {
    type Model: Model;

    fn new(model: &Self::Model, version: Version) -> Self;

    fn apply(self, state: &mut Self::Model);
}

pub struct Record<M: Model> {
    id: ID<M>,
    state: M,
    version: Version,
}

impl<M: Model> Record<M> {
    pub fn new(id: ID<M>, state: M, version: Version) -> Self {
        Self { id, state, version }
    }

    pub fn id(&self) -> ID<M> {
        self.id.clone()
    }

    pub fn state(&self) -> &M {
        &self.state
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn update<U: RecordUpdate<Model = M>>(
        &mut self,
        author: Ulid,
        callback: impl for<'r> Fn(&'r mut U) -> Result<(), crate::Error>,
    ) -> Result<bool, crate::Error> {
        let version = Version::now(author);
        let state = U::new(self.state(), version);
        let mut update = state.clone();

        callback(&mut update)?;
        if state != update {
            update.apply(&mut self.state);
            self.version = version;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn update_async<U: RecordUpdate<Model = M>, Fut: std::future::Future<Output = Result<(), crate::Error>>>(
        &mut self,
        author: Ulid,
        callback: impl for<'r> Fn(&'r mut U) -> Fut,
    ) -> Result<bool, crate::Error> {
        let version = Version::now(author);
        let state = U::new(self.state(), version);
        let mut update = state.clone();

        callback(&mut update).await?;
        if state != update {
            update.apply(&mut self.state);
            self.version = version;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
