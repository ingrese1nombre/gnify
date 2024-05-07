use chrono::NaiveDateTime;
use futures_lite::FutureExt;
use sqlx::{postgres::PgPoolOptions, types::Uuid, PgConnection, PgPool};
use ulid::Ulid;

use crate::{
    error::{InvalidValue, PersistenceError},
    vo::Version,
};

use super::Source;

pub struct PgSource(PgPool);

impl PgSource {
    pub async fn new(url: &str) -> Result<Self, PersistenceError> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(PgSource(pool))
    }

    pub async fn execute<Fut: std::future::Future<Output = crate::error::Result<()>> + Send>(&self, callback: impl for<'r> FnOnce(&'r mut PgConnection) -> Fut + Send) -> crate::error::Result<()> {
        let mut tx = self.0.begin().await?;
        callback(&mut tx).boxed().await?;
        tx.commit().await?;
        Ok(())
    }
}

impl Source for PgSource {
    type Connection<'r> = &'r mut PgConnection;

    async fn read<BMC: super::Read<Self> + Send>(
        &self,
        bmc: BMC,
    ) -> Result<BMC::Output, crate::error::PersistenceError> {
        let mut connection = self.0.acquire().await?;
        bmc.read(&mut connection).boxed().await
    }

    async fn write<BMC: super::Write<Self>>(&self, bmc: BMC) -> Result<(), PersistenceError> {
        let mut tx = self.0.begin().await?;
        bmc.write(&mut tx).boxed().await?;
        tx.commit().await?;
        Ok(())
    }
}

pub async fn add_corrupt_record(connection: &mut PgConnection, id: Uuid, model: &'static str, error: InvalidValue) -> Result<(), PersistenceError> {
    sqlx::query!(
        r#"
        insert into corrupt_record (id, model, description) 
        values ($1, $2, $3) on conflict (id) do nothing;
        "#,
        id,
        model,
        error.to_string()
    ).execute(connection).await?;
    Ok(())
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "version")]
pub struct RecordVersion {
    pub author: Uuid,
    pub timestamp: NaiveDateTime,
}

impl From<Version> for RecordVersion {
    fn from(value: Version) -> Self {
        RecordVersion {
            author: Uuid::from(value.author()),
            timestamp: value.timestamp(),
        }
    }
}

impl TryFrom<RecordVersion> for Version {
    type Error = InvalidValue;

    fn try_from(value: RecordVersion) -> Result<Self, Self::Error> {
        Version::new(Ulid::from(value.author), value.timestamp)
    }
}

impl From<sqlx::Error> for PersistenceError {
    fn from(value: sqlx::Error) -> Self {
        PersistenceError::new(value.to_string())
    }
}
impl From<sqlx::Error> for crate::Error {
    fn from(value: sqlx::Error) -> Self {
        PersistenceError::new(value.to_string()).into()
    }
}
