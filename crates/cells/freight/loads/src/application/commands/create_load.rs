use crate::domain::api::create_load::{
    AddressResource, CreateLoad, CreateLoadError, CreateLoadInput, LoadResource, StopKindPl,
    StopResource, ViolationDto, decode, decode_outcome, encode_outcome, fingerprint,
};
use crate::domain::entities::load::{Address, Load, Stop, StopKind};
use crate::domain::spi::load_events::LoadEvents;
use crate::domain::spi::load_store::{
    IdempotencyRecord, IdempotencyStore, LoadStore, LoadStoreError,
};
use kernel::{Clock, Logger};
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct CreateLoadCommand<S, E, C, Lg> {
    pub(crate) store: S,
    pub(crate) events: E,
    pub(crate) clock: C,
    pub(crate) logger: Lg,
}

impl<S, E, C, Lg> CreateLoad for CreateLoadCommand<S, E, C, Lg>
where
    S: LoadStore + IdempotencyStore + Sync,
    E: LoadEvents + Sync,
    C: Clock,
    Lg: Logger,
{
    async fn create_load(
        &self,
        actor_id: String,
        idempotency_key: String,
        input: CreateLoadInput,
    ) -> Result<LoadResource, CreateLoadError> {
        let input = decode(input)?;
        let fingerprint = fingerprint(&input);
        match self.store.get(&actor_id, &idempotency_key).await {
            Ok(Some(record)) if record.fingerprint == fingerprint => {
                return replay(&self.logger, &record.outcome);
            }
            Ok(Some(_)) => return Err(mismatch()),
            Ok(None) => {}
            Err(_) => unexpected(&self.logger),
        }

        let mut load = build_load(&actor_id, self.clock.now(), &input)?;
        let resource = to_resource(&load);
        let record = IdempotencyRecord {
            actor_id,
            key: idempotency_key,
            fingerprint,
            outcome: encode_outcome(&Ok(resource.clone())),
        };
        match self.store.save(&load, Some(record)).await {
            Ok(()) => {
                self.events.publish(load.pull_events()).await;
                Ok(resource)
            }
            Err(LoadStoreError::Conflict) => Err(CreateLoadError::LoadConflict),
            Err(LoadStoreError::Unexpected) => unexpected(&self.logger),
        }
    }
}

fn replay<Lg: Logger>(logger: &Lg, outcome: &str) -> Result<LoadResource, CreateLoadError> {
    match decode_outcome(outcome) {
        Ok(Ok(resource)) => Ok(resource),
        Ok(Err(e)) => Err(e),
        Err(()) => unexpected(logger),
    }
}

fn unexpected(logger: &impl Logger) -> ! {
    logger.error("loads.create_load.failed", &[("error", "unexpected")]);
    panic!("loads.create_load: unexpected store error");
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
) -> Result<Load, CreateLoadError> {
    let pickup = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Pickup)
        .ok_or(CreateLoadError::LoadInvalid)?;
    let delivery = input
        .stops
        .iter()
        .find(|stop| stop.kind == StopKindPl::Delivery)
        .ok_or(CreateLoadError::LoadInvalid)?;
    Load::create(
        mint(),
        input.shipper_id.clone(),
        actor_id.to_owned(),
        created_at,
        to_stop(pickup, StopKind::Pickup),
        to_stop(delivery, StopKind::Delivery),
    )
    .map_err(|_| CreateLoadError::LoadInvalid)
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
    use super::CreateLoadCommand;
    use crate::domain::api::create_load::{
        AddressInput, CreateLoad, CreateLoadError, CreateLoadInput, StopInput, StopKindPl,
    };
    use crate::domain::events::LoadEvent;
    use crate::domain::spi::load_events::FakeLoadEvents;
    use crate::domain::spi::load_store::{IdempotencyStore, LoadStore};
    use crate::infrastructure::load_store::SeaOrmLoadStore;
    use kernel::{FakeClock, Instant, Logger};
    use uuid::Uuid;

    struct Silent;

    impl Logger for Silent {
        fn debug(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn info(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn warn(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn error(&self, _msg: &str, _fields: &[(&str, &str)]) {}
    }

    async fn command() -> CreateLoadCommand<SeaOrmLoadStore, FakeLoadEvents, FakeClock, Silent> {
        let store = SeaOrmLoadStore::new(crate::infrastructure::test_db::migrated_pool().await);
        CreateLoadCommand {
            store,
            events: FakeLoadEvents::new(),
            clock: clock(),
            logger: Silent,
        }
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
        let command = command().await;
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let resource = command
            .create_load(actor, key, valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("create"));
        assert_eq!(resource.shipper_id, "shipper-1");
        assert_eq!(resource.created_at, "2023-11-14T22:13:20.000Z");
        assert_eq!(resource.stops.len(), 2);
        assert!(Uuid::parse_str(&resource.id).is_ok());
        let loaded = command
            .store
            .get_by_id(&resource.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(loaded.id, resource.id);
        assert_eq!(
            command.events.published(),
            vec![LoadEvent::Created {
                load_id: resource.id
            }]
        );
    }

    #[tokio::test]
    async fn create_load_decode_error_does_not_write() {
        let command = command().await;
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let mut input = valid_input("shipper-1");
        input.shipper_id.clear();
        match command.create_load(actor.clone(), key.clone(), input).await {
            Err(CreateLoadError::ValidationFailed { .. }) => {}
            other => panic!("{other:?}"),
        }
        assert_eq!(command.store.get(&actor, &key).await.unwrap(), None);
        assert_eq!(command.events.published(), vec![]);
    }

    #[tokio::test]
    async fn create_load_replays_the_stored_result() {
        let command = command().await;
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let first = command
            .create_load(actor.clone(), key.clone(), valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("first"));
        command
            .clock
            .set(command.clock.now().checked_add_seconds(60).unwrap());
        let second = command
            .create_load(actor, key, valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("replay"));
        assert_eq!(first, second);
        assert_eq!(first.created_at, "2023-11-14T22:13:20.000Z");
        assert_eq!(
            command.events.published(),
            vec![LoadEvent::Created { load_id: first.id }]
        );
    }

    #[tokio::test]
    async fn create_load_rejects_fingerprint_mismatch() {
        let command = command().await;
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        command
            .create_load(actor.clone(), key.clone(), valid_input("shipper-1"))
            .await
            .unwrap_or_else(|_| panic!("first"));
        match command
            .create_load(actor, key, valid_input("shipper-2"))
            .await
        {
            Err(CreateLoadError::ValidationFailed { violations }) => {
                assert_eq!(violations[0].code, "mismatch");
            }
            other => panic!("{other:?}"),
        }
    }

    #[tokio::test]
    async fn create_load_unique_key_conflict_or_replay() {
        let command = command().await;
        let actor = Uuid::now_v7().to_string();
        let key = Uuid::now_v7().to_string();
        let (a, b) = tokio::join!(
            command.create_load(actor.clone(), key.clone(), valid_input("shipper-1"),),
            command.create_load(actor.clone(), key.clone(), valid_input("shipper-1"),)
        );
        match (a, b) {
            (Ok(left), Ok(right)) => assert_eq!(left.id, right.id),
            (Ok(won), Err(CreateLoadError::LoadConflict))
            | (Err(CreateLoadError::LoadConflict), Ok(won)) => {
                let replayed = command
                    .create_load(actor, key, valid_input("shipper-1"))
                    .await
                    .unwrap_or_else(|_| panic!("retry"));
                assert_eq!(replayed.id, won.id);
            }
            other => panic!("{other:?}"),
        }
    }
}
