mod get {
    use gnify::{error::InvalidValue, source::{add_corrupt_record, PgSource, Read, RecordVersion}};
    use sqlx::types::Uuid;

    use crate::role::{bmc::GetRole, view::DetailedRoleView, RoleLevel};

    impl Read<PgSource> for GetRole {
        async fn read(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<Self::Output, gnify::error::PersistenceError> {
            let id = self.id.map(Uuid::from);
            let row: Option<RoleRow> = sqlx::query_as!(
                RoleRow,
                r#"
                select
                      r.id
                    , r.version as "version: RecordVersion"
                    , r.first_version "first_version: RecordVersion"
                    , r.name
                    , r.level
                    , array (
                        select privilege from core.role_privilege where role_id = r.id
                    ) as "privileges!"
                from core.role r
                    left join corrupt_record crec on crec.id = r.id
                where crec.id is null and (
                    r.id is not distinct from $1 or
                    r.name is not distinct from $2
                ) limit 1;
                "#,
                id,
                self.name
            )
            .fetch_optional(&mut *connection)
            .await?;
            let Some(row) = row else {
                return Ok(None);
            };
            let id = row.id;
            match map_row(row) {
                Ok(view) => Ok(Some(view)),
                Err(iv) => {
                    add_corrupt_record(connection, id, "core.role", iv).await?;
                    Ok(None)
                },
            }
        }
    }

    struct RoleRow {
        id: Uuid,
        version: RecordVersion,
        first_version: RecordVersion,
        name: String,
        level: i16,
        privileges: Vec<String>,
    }

    fn map_row(row: RoleRow) -> Result<DetailedRoleView, InvalidValue> {
        Ok(DetailedRoleView {
            id: row.id.into(),
            version: row.version.try_into()?,
            first_version: row.first_version.try_into()?,
            name: row.name.parse()?,
            level: RoleLevel::from(row.level),
            privileges: row.privileges.into_iter().map(|privilege| privilege.parse()).collect::<Result<_, InvalidValue>>()?,
        })
    }
}
mod write {
    use gnify::source::{PgSource, RecordVersion, Write};
    use sqlx::types::Uuid;

    use crate::role::{bmc::WriteRole, Role};

    impl Write<PgSource> for WriteRole {
        async fn write(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<(), gnify::error::PersistenceError> {
            let record = self.record;
            let id = Uuid::from(record.id());
            let version = RecordVersion::from(record.version());
            let Role { name, level, privileges } = record.state();
            let name = name.to_string();
            let level = *level as i16;
            let (ids, privileges): (Vec<Uuid>, Vec<String>) = privileges.iter().map(|privilege| (id, privilege.to_string())).unzip();
            sqlx::query!(
                r#"
                merge into core.role r
                using (values ($1::uuid, $2::version, $3, $4::smallint)) as src(id, version, name, level)
                on r.id = src.id 
                when not matched then
                    insert (id, version, first_version, name, level)
                    values (src.id, src.version, src.version, src.name, src.level)
                when matched then 
                    update set
                        version = src.version,
                        name = src.name,
                        level = src.level;
                "#,
                id,
                version as RecordVersion,
                name,
                level
            ).execute(&mut *connection).await?;
            sqlx::query!(
                r#"
                with cte as (
                    delete from core.role_privilege where role_id = $1 and privilege != all($3::text[]) 
                )
                insert into core.role_privilege (role_id, privilege) select * from unnest($2::uuid[], $3::text[])
                    on conflict (role_id, privilege) do nothing;
                "#,
                id,
                &ids[..],
                &privileges[..],
            ).execute(&mut *connection).await?;
            Ok(())
        }
    }
}