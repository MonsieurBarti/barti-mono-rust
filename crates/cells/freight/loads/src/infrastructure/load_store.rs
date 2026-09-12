use crate::domain::entities::load::{Load, LoadRow, StopRow, from_rows, to_rows};
use crate::domain::spi::load_store::{
    IdempotencyRecord, IdempotencyStore, LoadStore, LoadStoreError,
};
use crate::infrastructure::LoadsPool;
use kernel::Instant;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct SqlxLoadStore {
    pool: LoadsPool,
}

impl SqlxLoadStore {
    pub(crate) fn new(pool: LoadsPool) -> Self {
        Self { pool }
    }
}

impl LoadStore for SqlxLoadStore {
    fn get_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Load>, LoadStoreError>> + Send {
        let pool = self.pool.inner().clone();
        let id = id.to_owned();
        async move {
            let id = parse_uuid(&id)?;
            let load = sqlx::query!(
                r#"
        SELECT
            id::text AS "id!",
            shipper_id,
            actor_id,
            (EXTRACT(EPOCH FROM created_at) * 1000)::bigint AS "created_at_millis!"
        FROM loads.load
        WHERE id = $1
        "#,
                id
            )
            .fetch_optional(&pool)
            .await
            .map_err(map_err)?;
            let Some(load) = load else {
                return Ok(None);
            };
            let stops = sqlx::query!(
                r#"
        SELECT
            id::text AS "id!",
            load_id::text AS "load_id!",
            kind,
            date::text AS "date!",
            name,
            line1,
            line2,
            city,
            region,
            postal_code,
            country
        FROM loads.stop
        WHERE load_id = $1
        "#,
                id
            )
            .fetch_all(&pool)
            .await
            .map_err(map_err)?;
            Ok(Some(from_rows(
                LoadRow {
                    id: load.id,
                    shipper_id: load.shipper_id,
                    actor_id: load.actor_id,
                    created_at: Instant::from_unix_timestamp(load.created_at_millis / 1000)
                        .expect("corrupt created_at"),
                },
                stops
                    .into_iter()
                    .map(|stop| StopRow {
                        id: stop.id,
                        load_id: stop.load_id,
                        kind: stop.kind,
                        date: stop.date,
                        name: stop.name,
                        line1: stop.line1,
                        line2: stop.line2,
                        city: stop.city,
                        region: stop.region,
                        postal_code: stop.postal_code,
                        country: stop.country,
                    })
                    .collect(),
            )))
        }
    }

    fn save(
        &self,
        load: &Load,
        idempotency: Option<IdempotencyRecord>,
    ) -> impl Future<Output = Result<(), LoadStoreError>> + Send {
        let pool = self.pool.inner().clone();
        let load = load.clone();
        async move {
            let (load_row, stop_rows) = to_rows(&load);
            let mut tx = pool.begin().await.map_err(map_err)?;
            sqlx::query!(
                r#"
        INSERT INTO loads.load (id, shipper_id, actor_id, created_at)
        VALUES (
            $1,
            $2,
            $3,
            TIMESTAMPTZ 'epoch' + $4::bigint * INTERVAL '1 millisecond'
        )
        "#,
                parse_uuid(&load_row.id)?,
                load_row.shipper_id,
                load_row.actor_id,
                load_row.created_at.unix_timestamp() * 1000
            )
            .execute(&mut *tx)
            .await
            .map_err(map_err)?;
            for stop in stop_rows {
                sqlx::query!(
                    r#"
            INSERT INTO loads.stop (
                id, load_id, kind, date, name, line1, line2, city, region, postal_code, country
            )
            VALUES (
                $1,
                $2,
                $3,
                to_date($4, 'YYYY-MM-DD'),
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11
            )
            "#,
                    parse_uuid(&stop.id)?,
                    parse_uuid(&stop.load_id)?,
                    stop.kind,
                    stop.date,
                    stop.name,
                    stop.line1,
                    stop.line2,
                    stop.city,
                    stop.region,
                    stop.postal_code,
                    stop.country
                )
                .execute(&mut *tx)
                .await
                .map_err(map_err)?;
            }
            if let Some(record) = idempotency {
                sqlx::query!(
                    r#"
            INSERT INTO loads.idempotency_key (actor_id, key, fingerprint, outcome)
            VALUES ($1, $2, $3, $4)
            "#,
                    record.actor_id,
                    record.key,
                    record.fingerprint,
                    record.outcome
                )
                .execute(&mut *tx)
                .await
                .map_err(map_err)?;
            }
            tx.commit().await.map_err(map_err)?;
            Ok(())
        }
    }
}

impl IdempotencyStore for SqlxLoadStore {
    fn get(
        &self,
        actor_id: &str,
        key: &str,
    ) -> impl Future<Output = Result<Option<IdempotencyRecord>, LoadStoreError>> + Send {
        let pool = self.pool.inner().clone();
        let actor_id = actor_id.to_owned();
        let key = key.to_owned();
        async move {
            let row = sqlx::query!(
                r#"
                SELECT actor_id, key, fingerprint, outcome
                FROM loads.idempotency_key
                WHERE actor_id = $1 AND key = $2
                "#,
                actor_id,
                key
            )
            .fetch_optional(&pool)
            .await
            .map_err(map_err)?;
            Ok(row.map(|row| IdempotencyRecord {
                actor_id: row.actor_id,
                key: row.key,
                fingerprint: row.fingerprint,
                outcome: row.outcome,
            }))
        }
    }
}

fn map_err(err: sqlx::Error) -> LoadStoreError {
    if let sqlx::Error::Database(db) = &err
        && db.code().as_deref() == Some("23505")
    {
        return LoadStoreError::Conflict;
    }
    LoadStoreError::Unexpected
}

fn parse_uuid(id: &str) -> Result<Uuid, LoadStoreError> {
    Uuid::parse_str(id).map_err(|_| LoadStoreError::Unexpected)
}

#[cfg(test)]
mod integration {
    use super::SqlxLoadStore;
    use crate::domain::entities::load::LoadBuilder;
    use crate::domain::spi::load_store::{IdempotencyRecord, load_store_contract};
    use crate::infrastructure::LoadsPool;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn sqlx_store() -> SqlxLoadStore {
        let migrator_url = std::env::var("LOADS_MIGRATOR_DATABASE_URL")
            .expect("LOADS_MIGRATOR_DATABASE_URL is required");
        let cell_url = std::env::var("LOADS_DATABASE_URL").expect("LOADS_DATABASE_URL is required");
        let migrator = PgPool::connect(&migrator_url)
            .await
            .expect("migrator DSN unreachable");
        sqlx::migrate!("./migrations")
            .run(&migrator)
            .await
            .expect("loads migrations");
        let pool = PgPool::connect(&cell_url)
            .await
            .expect("cell-role DSN unreachable");
        SqlxLoadStore::new(LoadsPool::new(pool))
    }

    fn unique_load(label: u8) -> crate::domain::entities::load::Load {
        LoadBuilder::new()
            .id(Uuid::now_v7().to_string())
            .pickup_id(Uuid::now_v7().to_string())
            .delivery_id(Uuid::now_v7().to_string())
            .pickup_name(format!("yard-{label}"))
            .build()
    }

    #[tokio::test]
    async fn sqlx_adapter_satisfies_load_store_contract() {
        let store = sqlx_store().await;
        let load = unique_load(1);
        let keyed = unique_load(2);
        load_store_contract(
            &store,
            &Uuid::now_v7().to_string(),
            &load,
            &keyed,
            IdempotencyRecord {
                actor_id: keyed.actor_id.clone(),
                key: Uuid::now_v7().to_string(),
                fingerprint: "fp-sqlx".to_owned(),
                outcome: "ok".to_owned(),
            },
        )
        .await;
    }
}
