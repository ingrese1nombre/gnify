mod list {
    use std::ops::Deref;

    use gnify::{
        error::InvalidValue,
        source::{PgSource, Read, RecordVersion},
        vo::Version,
    };
    use serde::Deserialize;
    use sqlx::types::{chrono::NaiveDateTime, Json, Uuid};

    use crate::device::{DeviceStatus, DeviceView, ListDevices, Session};

    impl Read<PgSource> for ListDevices {
        async fn read(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<Self::Output, gnify::error::PersistenceError> {
            let status = self.status.map(|status| status as i16);
            sqlx::query!(
                r#"
                delete from core.session where expiration <= CURRENT_TIMESTAMP;
                "#
            )
            .execute(&mut *connection)
            .await?;
            let rows: Vec<DeviceRow> = sqlx::query_as!(
                DeviceRow,
                r#"
                select
                    d.token,
                    d.version as "version: RecordVersion",
                    d.first_version as "first_version: RecordVersion",
                    d.name,
                    d.status,
                    (
                        select
                            row_to_json(s)
                        from core.session s
                        where id = d.session_id
                        limit 1
                    ) as "session!: Option<Json<SessionRow>>"
                from core.device d
                where coalesce(d.status = $1, true);
                "#,
                status
            )
            .fetch_all(&mut *connection)
            .await?;

            let (devices, corrupt_devices) = map_rows(rows);

            if !corrupt_devices.is_empty() {
                sqlx::query!(
                    r#"
                    delete from core.device where token = any($1::text[]);
                    "#,
                    &corrupt_devices[..]
                )
                .execute(connection)
                .await?;
            }

            Ok(devices)
        }
    }

    struct DeviceRow {
        token: String,
        version: RecordVersion,
        first_version: RecordVersion,
        name: String,
        session: Option<Json<SessionRow>>,
        status: DeviceStatus,
    }

    #[derive(Deserialize)]
    struct SessionRow {
        token: String,
        user_id: Uuid,
        expiration: NaiveDateTime,
    }

    fn map_rows(rows: impl IntoIterator<Item = DeviceRow>) -> (Vec<DeviceView>, Vec<String>) {
        fn map_session(session: impl Deref<Target = SessionRow>) -> Option<Session> {
            Some(Session {
                token: session.token.parse().ok()?,
                user_id: session.user_id.into(),
                expiration: session.expiration.into(),
            })
        }
        fn map_device(device: DeviceRow) -> Result<DeviceView, InvalidValue> {
            let session = device.session.and_then(map_session);
            Ok(DeviceView {
                token: device.token.parse()?,
                version: Version::try_from(device.version)?,
                first_version: Version::try_from(device.first_version)?,
                name: device.name.parse()?,
                session,
                status: device.status,
            })
        }
        rows.into_iter()
            .fold((Vec::new(), Vec::new()), |mut acc, row| {
                let token = row.token.clone();
                match map_device(row) {
                    Ok(device) => acc.0.push(device),
                    Err(_) => acc.1.push(token),
                }
                acc
            })
    }
}
mod write {
    use gnify::source::{PgSource, RecordVersion, Write};
    use sqlx::types::chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::device::{Device, WriteDevice};

    impl Write<PgSource> for WriteDevice {
        async fn write(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<(), gnify::error::PersistenceError> {
            let record = self.record;
            let token = record.id().to_string();
            let version = RecordVersion::from(record.version());

            let Device { name, session, status } = record.state();
            let name = name.to_string();
            let status = *status as i16;
            sqlx::query!(
                r#"
                merge into core.device d
                using (values ($1, $2::version, $3, $4::smallint))
                    as src(token, version, name, status)
                on d.token = src.token
                when not matched then
                    insert (token, version, first_version, name, status)
                    values (src.token, src.version, src.version, src.name, src.status)
                when matched then
                    update set 
                        version = src.version,
                        name = src.name,
                        status = src.status;
                "#,
                token,
                version as RecordVersion,
                name,
                status
            ).execute(&mut *connection).await?;
            match session {
                Some(session) => {
                    let session_token = session.token.to_string();
                    let user_id = Uuid::from(session.user_id.value());
                    let expiration = NaiveDateTime::from(session.expiration);
                    sqlx::query!(
                        r#"
                        with cte as (
                            delete from core.session where token = $4
                        ),
                        s as (
                            insert into core.session(token, user_id, expiration)
                            values ($1, $2::uuid, $3) returning id
                        )
                        update core.device set 
                            session_id = (select id from s limit 1);
                        "#,
                        session_token,
                        user_id,
                        expiration,
                        token
                    ).execute(connection).await?;
                },
                None => {
                    sqlx::query!(
                        r#"
                        delete from core.session where token = $1
                        "#,
                        token
                    ).execute(connection).await?;
                },
            }
            Ok(())
        }
    }
}
