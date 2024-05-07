mod get {
    use std::collections::HashSet;

    use gnify::{
        error::InvalidValue,
        source::{PgSource, Read, RecordVersion},
        vo::{Version, ID},
    };
    use sqlx::types::Uuid;

    use crate::{
        role::RoleLevel,
        user::{
            bmc::GetUser,
            view::{DetailedUserView, UserRole},
        },
        Privilege,
    };

    impl Read<PgSource> for GetUser {
        async fn read(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<Self::Output, gnify::error::PersistenceError> {
            let id = self.id.map(Uuid::from);
            let user_row: Option<UserRow> = sqlx::query_as!(
                UserRow,
                r#"
                select 
                    u.id
                    , u.version as "version: RecordVersion"
                    , u.first_version as "first_version: RecordVersion"
                    , u.username
                    , u.email
                    , u.password
                    , u.role_id
                    , array(select privilege from core.user_privilege where user_id = u.id) as "privileges!"
                from core.user u
                    left join public.corrupt_record crec on u.id = crec.id
                where crec.id is null and (
                    u.id is not distinct from $1 or
                    u.username is not distinct from $2 or 
                    ($3::text is not null and u.email is not distinct from $3::text)
                );
                "#,
                id,
                self.username,
                self.email
            ).fetch_optional(&mut *connection)
                .await?;
            

            let Some(user_row) = user_row else {
                return Ok(None);
            };

            let role_row: Option<RoleRow> = sqlx::query_as!(
            RoleRow,
            r#"
            select 
                r.id
                , r.name
                , r.level
                , array(select privilege from core.role_privilege where role_id = r.id) as "privileges!"
            from core.role r
                left join public.corrupt_record crec on r.id = crec.id
            where crec.id is null and r.id = $1;
            "#,
            user_row.role_id
        )
        .fetch_optional(&mut *connection)
        .await?;
            let id = user_row.id;
            match map_rows(user_row, role_row) {
                Ok(view) => Ok(Some(view)),
                Err(iv) => {
                    sqlx::query!(
                    r#"
                    insert into public."corrupt_record" (id, model, description) values ($1, 'core.user', $2) on conflict (id) do nothing;
                    "#,
                    id,
                    iv.to_string() as String
                ).execute(&mut *connection).await?;
                    Ok(None)
                }
            }
        }
    }

    #[derive(Debug)]
    struct UserRow {
        id: Uuid,
        version: RecordVersion,
        first_version: RecordVersion,
        username: String,
        email: Option<String>,
        password: String,
        privileges: Vec<String>,
        role_id: Uuid,
    }

    struct RoleRow {
        id: Uuid,
        name: String,
        level: i16,
        privileges: Vec<String>,
    }

    fn map_rows(
        user_row: UserRow,
        role_row: Option<RoleRow>,
    ) -> Result<DetailedUserView, InvalidValue> {
        let privileges = user_row
            .privileges
            .into_iter()
            .map(|value| value.parse())
            .collect::<Result<HashSet<Privilege>, InvalidValue>>()?;
        let role = if let Some(row) = role_row {
            Some(UserRole {
                id: ID::from(row.id),
                name: row.name.parse()?,
                level: RoleLevel::from(row.level),
                privileges: row
                    .privileges
                    .into_iter()
                    .map(|value| value.parse())
                    .collect::<Result<HashSet<Privilege>, InvalidValue>>()?,
            })
        } else {
            None
        };
        Ok(DetailedUserView {
            id: ID::from(user_row.id),
            version: Version::try_from(user_row.version)?,
            first_version: Version::try_from(user_row.first_version)?,
            username: user_row.username.parse()?,
            email: user_row.email.map(|value| value.parse()).transpose()?,
            password: user_row.password.parse()?,
            privileges,
            role,
        })
    }
}
mod write {
    use gnify::source::{PgSource, RecordVersion, Write};
    use sqlx::types::Uuid;

    use crate::user::{bmc::WriteUser, User};

    impl Write<PgSource> for WriteUser {
        async fn write(
            self,
            connection: <PgSource as gnify::source::Source>::Connection<'_>,
        ) -> Result<(), gnify::error::PersistenceError> {
            let record = self.record;
            let id = Uuid::from(*record.id());
            let version = RecordVersion::from(record.version());
            let User {
                username,
                password,
                email,
                role_id,
                privileges,
            } = record.state();
            let username = username.to_string();
            let password = password.to_string();
            let email = email.as_ref().map(ToString::to_string);
            let role_id = role_id.map(Uuid::from);
            sqlx::query!(
                r#"
                merge into core.user as u
                using (values ($1::uuid, $2::version, $3, $4, $5, $6::uuid)) as src(id, version, username, "password", email, role_id)
                on u.id = src.id
                when not matched then 
                    insert (id, version, first_version, username, password, email, role_id) 
                    values (src.id, src.version, src.version, src.username, src.password, src.email, src.role_id)
                when matched then
                    update set 
                        version = src.version,
                        username = src.username,
                        password = src.password,
                        email = src.email,
                        role_id = src.role_id;
                "#,
                id,
                version as RecordVersion,
                username,
                password,
                email,
                role_id
            ).execute(&mut *connection).await?;
            let (ids, privileges): (Vec<Uuid>, Vec<String>) = privileges
                .iter()
                .map(|privilege| (id, privilege.to_string()))
                .unzip();
            sqlx::query!(
                r#"
                with cte as (
                    delete from core.user_privilege where user_id = $1::uuid and privilege != all($3::text[])
                )
                insert into core.user_privilege(user_id, privilege) 
                select * from unnest($2::uuid[], $3::text[]) 
                    on conflict (user_id, privilege) do nothing
                "#,
                id,
                &ids[..],
                &privileges[..]
            ).execute(&mut *connection).await?;
            Ok(())
        }
    }
}
