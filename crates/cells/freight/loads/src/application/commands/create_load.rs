use crate::domain::api::create_load::{
    AddressResource, CreateLoadError, CreateLoadInput, LoadResource, StopKindPl, StopResource,
    ViolationDto, decode, decode_outcome, encode_outcome, fingerprint,
};
use crate::domain::entities::load::{Address, Load, Stop, StopKind};
use crate::domain::spi::load_store::{
    IdempotencyRecord, IdempotencyStore, LoadStore, LoadStoreError,
};
use kernel::Clock;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) enum CreateLoadFailure {
    Envelope(CreateLoadError),
    Unexpected,
}

pub(crate) async fn run<S, C>(
    store: &S,
    clock: &C,
    actor_id: String,
    idempotency_key: String,
    input: CreateLoadInput,
) -> Result<LoadResource, CreateLoadFailure>
where
    S: LoadStore + IdempotencyStore,
    C: Clock,
{
    let input = decode(input).map_err(CreateLoadFailure::Envelope)?;
    let fingerprint = fingerprint(&input);
    match store.get(&actor_id, &idempotency_key).await {
        Ok(Some(record)) if record.fingerprint == fingerprint => {
            return replay(&record.outcome);
        }
        Ok(Some(_)) => {
            return Err(CreateLoadFailure::Envelope(mismatch()));
        }
        Ok(None) => {}
        Err(LoadStoreError::Conflict) => {
            return Err(CreateLoadFailure::Envelope(CreateLoadError::LoadConflict));
        }
        Err(LoadStoreError::Unexpected) => return Err(CreateLoadFailure::Unexpected),
    }

    let load = match build_load(&actor_id, clock.now(), &input) {
        Ok(load) => load,
        Err(()) => return Err(CreateLoadFailure::Unexpected),
    };
    let resource = to_resource(&load);
    let record = IdempotencyRecord {
        actor_id,
        key: idempotency_key,
        fingerprint,
        outcome: encode_outcome(&Ok(resource.clone())),
    };
    match store.save(&load, Some(record)).await {
        Ok(()) => Ok(resource),
        Err(LoadStoreError::Conflict) => {
            Err(CreateLoadFailure::Envelope(CreateLoadError::LoadConflict))
        }
        Err(LoadStoreError::Unexpected) => Err(CreateLoadFailure::Unexpected),
    }
}

fn replay(outcome: &str) -> Result<LoadResource, CreateLoadFailure> {
    match decode_outcome(outcome) {
        Ok(Ok(resource)) => Ok(resource),
        Ok(Err(error)) => Err(CreateLoadFailure::Envelope(error)),
        Err(()) => Err(CreateLoadFailure::Unexpected),
    }
}

fn mismatch() -> CreateLoadError {
    CreateLoadError::ValidationFailed {
        violations: vec![ViolationDto {
            path: "Idempotency-Key".to_owned(),
            code: "mismatch".to_owned(),
            message: "does not match the original request".to_owned(),
        }],
    }
}

fn build_load(
    actor_id: &str,
    created_at: kernel::Instant,
    input: &CreateLoadInput,
) -> Result<Load, ()> {
    let pickup = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Pickup)
        .ok_or(())?;
    let delivery = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Delivery)
        .ok_or(())?;
    Load::create(
        mint(),
        input.shipper_id.clone(),
        actor_id.to_owned(),
        created_at,
        to_stop(pickup, StopKind::Pickup),
        to_stop(delivery, StopKind::Delivery),
    )
    .map_err(|_| ())
}

fn to_stop(input: &crate::domain::api::create_load::StopInput, kind: StopKind) -> Stop {
    Stop {
        id: mint(),
        kind,
        date: input.date.clone(),
        name: input.name.clone(),
        address: Address {
            line1: input.address.line1.clone(),
            line2: input.address.line2.clone(),
            city: input.address.city.clone(),
            region: input.address.region.clone(),
            postal_code: input.address.postal_code.clone(),
            country: input.address.country.clone(),
        },
    }
}

fn to_resource(load: &Load) -> LoadResource {
    LoadResource {
        id: load.id.clone(),
        shipper_id: load.shipper_id.clone(),
        created_at: load.created_at.to_rfc3339_millis(),
        stops: load
            .stops
            .iter()
            .map(|stop| StopResource {
                id: stop.id.clone(),
                kind: match stop.kind {
                    StopKind::Pickup => StopKindPl::Pickup,
                    StopKind::Delivery => StopKindPl::Delivery,
                },
                date: stop.date.clone(),
                name: stop.name.clone(),
                address: AddressResource {
                    line1: stop.address.line1.clone(),
                    line2: stop.address.line2.clone(),
                    city: stop.address.city.clone(),
                    region: stop.address.region.clone(),
                    postal_code: stop.address.postal_code.clone(),
                    country: stop.address.country.clone(),
                },
            })
            .collect(),
    }
}

fn mint() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod integration {
    use super::{CreateLoadFailure, run};
    use crate::domain::api::create_load::{
        AddressInput, CreateLoadError, CreateLoadInput, StopInput, StopKindPl,
    };
    use crate::domain::spi::load_store::{IdempotencyStore, LoadStore};
    use crate::infrastructure::LoadsPool;
    use crate::infrastructure::load_store::SqlxLoadStore;
    use kernel::{FakeClock, Instant};
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

    fn clock() -> FakeClock {
        FakeClock::new(Instant::from_unix_timestamp(1_700_000_000).unwrap())
    }

    fn address() -> AddressInput {
        AddressInput {
            line1: "1 Dock".to_owned(),
            line2: None,
            city: "Dallas".to_owned(),
            region: "TX".to_owned(),
            postal_code: "75201".to_owned(),
            country: "US".to_owned(),
        }
    }

    fn valid_input(shipper_id: &str) -> CreateLoadInput {
        CreateLoadInput {
            shipper_id: shipper_id.to_owned(),
            stops: vec![
                StopInput {
                    kind: StopKindPl::Pickup,
                    date: "2026-09-20".to_owned(),
                    name: None,
                    address: address(),
                },
                StopInput {
                    kind: StopKindPl::Delivery,
                    date: "2026-09-21".to_owned(),
                    name: Some("Consignee".to_owned()),
                    address: AddressInput {
                        line1: "9 Warehouse".to_owned(),
                        line2: None,
                        city: "Austin".to_owned(),
                        region: "TX".to_owned(),
                        postal_code: "78701".to_owned(),
                        country: "US".to_owned(),
                    },
                },
            ],
        }
    }

    #[tokio::test]
    async fn create_load_persists_and_returns_minted_ids() {
        let store = sqlx_store().await;
        let clock = clock();
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let resource = run(&store, &clock, actor, key, valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("create"));
        assert_eq!(resource.shipper_id, "shipper-1");
        assert_eq!(resource.created_at, "2023-11-14T22:13:20.000Z");
        assert_eq!(resource.stops.len(), 2);
        assert!(Uuid::parse_str(&resource.id).is_ok());
        let loaded = store.get_by_id(&resource.id).await.unwrap().unwrap();
        assert_eq!(loaded.id, resource.id);
    }

    #[tokio::test]
    async fn create_load_decode_error_does_not_write() {
        let store = sqlx_store().await;
        let clock = clock();
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let mut input = valid_input("shipper-1");
        input.shipper_id.clear();
        match run(&store, &clock, actor.clone(), key.clone(), input).await {
            Err(CreateLoadFailure::Envelope(CreateLoadError::ValidationFailed { .. })) => {}
            other => panic!("{other:?}"),
        }
        assert_eq!(store.get(&actor, &key).await.unwrap(), None);
    }

    #[tokio::test]
    async fn create_load_replays_the_stored_result() {
        let store = sqlx_store().await;
        let clock = clock();
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let first = run(
            &store,
            &clock,
            actor.clone(),
            key.clone(),
            valid_input("shipper-1"),
        )
        .await
        .unwrap_or_else(|_| panic!("first"));
        clock.set(clock.now().checked_add_seconds(60).unwrap());
        let second = run(&store, &clock, actor, key, valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("replay"));
        assert_eq!(first, second);
        assert_eq!(first.created_at, "2023-11-14T22:13:20.000Z");
    }

    #[tokio::test]
    async fn create_load_rejects_fingerprint_mismatch() {
        let store = sqlx_store().await;
        let clock = clock();
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        run(
            &store,
            &clock,
            actor.clone(),
            key.clone(),
            valid_input("shipper-1"),
        )
        .await
        .unwrap_or_else(|_| panic!("first"));
        match run(&store, &clock, actor, key, valid_input("shipper-2")).await {
            Err(CreateLoadFailure::Envelope(CreateLoadError::ValidationFailed { violations })) => {
                assert_eq!(violations[0].code, "mismatch");
            }
            other => panic!("{other:?}"),
        }
    }

    #[tokio::test]
    async fn create_load_unique_key_conflict_or_replay() {
        let store = sqlx_store().await;
        let clock = clock();
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let (a, b) = tokio::join!(
            run(
                &store,
                &clock,
                actor.clone(),
                key.clone(),
                valid_input("shipper-1"),
            ),
            run(
                &store,
                &clock,
                actor.clone(),
                key.clone(),
                valid_input("shipper-1"),
            )
        );
        match (a, b) {
            (Ok(left), Ok(right)) => assert_eq!(left.id, right.id),
            (Ok(won), Err(CreateLoadFailure::Envelope(CreateLoadError::LoadConflict)))
            | (Err(CreateLoadFailure::Envelope(CreateLoadError::LoadConflict)), Ok(won)) => {
                let replayed = run(&store, &clock, actor, key, valid_input("shipper-1"))
                    .await
                    .unwrap_or_else(|_| panic!("retry"));
                assert_eq!(replayed.id, won.id);
            }
            other => panic!("{other:?}"),
        }
    }
}
