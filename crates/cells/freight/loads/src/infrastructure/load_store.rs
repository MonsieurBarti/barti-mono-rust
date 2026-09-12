use crate::domain::entities::load::{Load, LoadRow, StopRow, from_rows, to_rows};
use crate::domain::spi::load_store::{
    IdempotencyRecord, IdempotencyStore, LoadStore, LoadStoreError,
};
use crate::infrastructure::LoadsPool;
use kernel::Instant;
use sea_orm::prelude::{TimeDate, TimeDateTimeWithTimeZone};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use uuid::Uuid;

pub(crate) mod load {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "loads", table_name = "load")]
    pub(crate) struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub(crate) id: Uuid,
        pub(crate) shipper_id: String,
        pub(crate) actor_id: String,
        pub(crate) created_at: TimeDateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub(crate) enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod stop {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "loads", table_name = "stop")]
    pub(crate) struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub(crate) id: Uuid,
        pub(crate) load_id: Uuid,
        pub(crate) kind: String,
        pub(crate) date: TimeDate,
        pub(crate) name: Option<String>,
        pub(crate) line1: String,
        pub(crate) line2: Option<String>,
        pub(crate) city: String,
        pub(crate) region: String,
        pub(crate) postal_code: String,
        pub(crate) country: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub(crate) enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub(crate) mod idempotency_key {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "loads", table_name = "idempotency_key")]
    pub(crate) struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub(crate) actor_id: String,
        #[sea_orm(primary_key, auto_increment = false)]
        pub(crate) key: String,
        pub(crate) fingerprint: String,
        pub(crate) outcome: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub(crate) enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

fn parse_uuid(id: &str) -> Result<Uuid, LoadStoreError> {
    Uuid::parse_str(id).map_err(|_| LoadStoreError::Unexpected)
}

fn to_timestamp(instant: Instant) -> Result<TimeDateTimeWithTimeZone, LoadStoreError> {
    TimeDateTimeWithTimeZone::from_unix_timestamp(instant.unix_timestamp())
        .map_err(|_| LoadStoreError::Unexpected)
}

fn from_timestamp(value: TimeDateTimeWithTimeZone) -> Instant {
    Instant::from_unix_timestamp(value.unix_timestamp()).expect("corrupt created_at")
}

fn to_date(ymd: &str) -> Result<TimeDate, LoadStoreError> {
    let (year, rest) = ymd.split_once('-').ok_or(LoadStoreError::Unexpected)?;
    let (month, day) = rest.split_once('-').ok_or(LoadStoreError::Unexpected)?;
    let year: i32 = year.parse().map_err(|_| LoadStoreError::Unexpected)?;
    let month: u8 = month.parse().map_err(|_| LoadStoreError::Unexpected)?;
    let day: u8 = day.parse().map_err(|_| LoadStoreError::Unexpected)?;
    TimeDate::from_calendar_date(
        year,
        TryFrom::try_from(month).map_err(|_| LoadStoreError::Unexpected)?,
        day,
    )
    .map_err(|_| LoadStoreError::Unexpected)
}

fn from_date(date: TimeDate) -> String {
    date.to_string()
}

fn load_active(row: &LoadRow) -> Result<load::ActiveModel, LoadStoreError> {
    Ok(load::ActiveModel {
        id: Set(parse_uuid(&row.id)?),
        shipper_id: Set(row.shipper_id.clone()),
        actor_id: Set(row.actor_id.clone()),
        created_at: Set(to_timestamp(row.created_at)?),
    })
}

fn stop_active(row: &StopRow) -> Result<stop::ActiveModel, LoadStoreError> {
    Ok(stop::ActiveModel {
        id: Set(parse_uuid(&row.id)?),
        load_id: Set(parse_uuid(&row.load_id)?),
        kind: Set(row.kind.clone()),
        date: Set(to_date(&row.date)?),
        name: Set(row.name.clone()),
        line1: Set(row.line1.clone()),
        line2: Set(row.line2.clone()),
        city: Set(row.city.clone()),
        region: Set(row.region.clone()),
        postal_code: Set(row.postal_code.clone()),
        country: Set(row.country.clone()),
    })
}

fn key_active(record: IdempotencyRecord) -> idempotency_key::ActiveModel {
    idempotency_key::ActiveModel {
        actor_id: Set(record.actor_id),
        key: Set(record.key),
        fingerprint: Set(record.fingerprint),
        outcome: Set(record.outcome),
    }
}

fn load_row(model: load::Model) -> LoadRow {
    LoadRow {
        id: model.id.to_string(),
        shipper_id: model.shipper_id,
        actor_id: model.actor_id,
        created_at: from_timestamp(model.created_at),
    }
}

fn stop_row(model: stop::Model) -> StopRow {
    StopRow {
        id: model.id.to_string(),
        load_id: model.load_id.to_string(),
        kind: model.kind,
        date: from_date(model.date),
        name: model.name,
        line1: model.line1,
        line2: model.line2,
        city: model.city,
        region: model.region,
        postal_code: model.postal_code,
        country: model.country,
    }
}

fn key_record(model: idempotency_key::Model) -> IdempotencyRecord {
    IdempotencyRecord {
        actor_id: model.actor_id,
        key: model.key,
        fingerprint: model.fingerprint,
        outcome: model.outcome,
    }
}

#[derive(Clone)]
pub(crate) struct SeaOrmLoadStore {
    pool: LoadsPool,
}

impl SeaOrmLoadStore {
    pub(crate) fn new(pool: LoadsPool) -> Self {
        Self { pool }
    }
}

impl LoadStore for SeaOrmLoadStore {
    fn get_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Load>, LoadStoreError>> + Send {
        let conn = self.pool.inner().clone();
        let id = id.to_owned();
        async move {
            let id = parse_uuid(&id)?;
            let Some(load) = load::Entity::find_by_id(id)
                .one(&conn)
                .await
                .map_err(map_err)?
            else {
                return Ok(None);
            };
            let stops = stop::Entity::find()
                .filter(stop::Column::LoadId.eq(id))
                .all(&conn)
                .await
                .map_err(map_err)?;
            Ok(Some(from_rows(
                load_row(load),
                stops.into_iter().map(stop_row).collect(),
            )))
        }
    }

    fn save(
        &self,
        load: &Load,
        idempotency: Option<IdempotencyRecord>,
    ) -> impl Future<Output = Result<(), LoadStoreError>> + Send {
        let conn = self.pool.inner().clone();
        let load = load.clone();
        async move {
            let (load_row, stop_rows) = to_rows(&load);
            let txn = conn.begin().await.map_err(map_err)?;
            load::Entity::insert(load_active(&load_row)?)
                .exec(&txn)
                .await
                .map_err(map_err)?;
            stop::Entity::insert_many(
                stop_rows
                    .iter()
                    .map(stop_active)
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .exec(&txn)
            .await
            .map_err(map_err)?;
            if let Some(record) = idempotency {
                idempotency_key::Entity::insert(key_active(record))
                    .exec(&txn)
                    .await
                    .map_err(map_err)?;
            }
            txn.commit().await.map_err(map_err)
        }
    }
}

impl IdempotencyStore for SeaOrmLoadStore {
    fn get(
        &self,
        actor_id: &str,
        key: &str,
    ) -> impl Future<Output = Result<Option<IdempotencyRecord>, LoadStoreError>> + Send {
        let conn = self.pool.inner().clone();
        let actor_id = actor_id.to_owned();
        let key = key.to_owned();
        async move {
            Ok(idempotency_key::Entity::find_by_id((actor_id, key))
                .one(&conn)
                .await
                .map_err(map_err)?
                .map(key_record))
        }
    }
}

fn map_err(err: sea_orm::DbErr) -> LoadStoreError {
    match err.sql_err() {
        Some(sea_orm::SqlErr::UniqueConstraintViolation(_)) => LoadStoreError::Conflict,
        _ => LoadStoreError::Unexpected,
    }
}

#[cfg(test)]
mod tests {
    use super::{load_active, load_row, stop_active, stop_row};
    use crate::domain::entities::load::{LoadRow, StopRow};
    use kernel::Instant;
    use sea_orm::TryIntoModel;

    fn sample_stop(date: &str) -> StopRow {
        StopRow {
            id: "01900000-0000-7000-8000-000000000012".to_owned(),
            load_id: "01900000-0000-7000-8000-000000000011".to_owned(),
            kind: "pickup".to_owned(),
            date: date.to_owned(),
            name: Some("yard".to_owned()),
            line1: "1 Dock".to_owned(),
            line2: Some("Suite 2".to_owned()),
            city: "Dallas".to_owned(),
            region: "TX".to_owned(),
            postal_code: "75201".to_owned(),
            country: "US".to_owned(),
        }
    }

    #[test]
    fn stop_row_round_trips_plain_date() {
        let row = sample_stop("2026-09-20");
        let model = stop_active(&row)
            .expect("active")
            .try_into_model()
            .expect("model");
        assert_eq!(stop_row(model), row);
    }

    #[test]
    fn stop_row_round_trips_leap_day() {
        let row = sample_stop("2028-02-29");
        let model = stop_active(&row)
            .expect("active")
            .try_into_model()
            .expect("model");
        assert_eq!(stop_row(model), row);
    }

    #[test]
    fn load_row_round_trips_created_at() {
        let row = LoadRow {
            id: "01900000-0000-7000-8000-000000000011".to_owned(),
            shipper_id: "shipper-1".to_owned(),
            actor_id: "actor-1".to_owned(),
            created_at: Instant::from_unix_timestamp(1_700_000_000).expect("t0"),
        };
        let model = load_active(&row)
            .expect("active")
            .try_into_model()
            .expect("model");
        assert_eq!(load_row(model), row);
    }
}

#[cfg(test)]
mod integration {
    use super::SeaOrmLoadStore;
    use crate::domain::entities::load::LoadBuilder;
    use crate::domain::spi::load_store::{IdempotencyRecord, load_store_contract};
    use uuid::Uuid;

    fn unique_load(label: u8) -> crate::domain::entities::load::Load {
        LoadBuilder::new()
            .id(Uuid::now_v7().to_string())
            .pickup_id(Uuid::now_v7().to_string())
            .delivery_id(Uuid::now_v7().to_string())
            .pickup_name(format!("yard-{label}"))
            .build()
    }

    #[tokio::test]
    async fn sea_orm_adapter_satisfies_load_store_contract() {
        let store = SeaOrmLoadStore::new(crate::infrastructure::test_db::migrated_pool().await);
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
                fingerprint: "fp-1".to_owned(),
                outcome: "ok".to_owned(),
            },
        )
        .await;
    }
}
