use crate::domain::events::LoadEvent;
use kernel::Instant;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum StopKind {
    Pickup,
    Delivery,
}

impl StopKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pickup => "pickup",
            Self::Delivery => "delivery",
        }
    }

    #[allow(dead_code)]
    fn parse(kind: &str) -> Self {
        match kind {
            "pickup" => Self::Pickup,
            "delivery" => Self::Delivery,
            other => panic!("corrupt stop kind {other}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Address {
    pub(crate) line1: String,
    pub(crate) line2: Option<String>,
    pub(crate) city: String,
    pub(crate) region: String,
    pub(crate) postal_code: String,
    pub(crate) country: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Stop {
    pub(crate) id: String,
    pub(crate) kind: StopKind,
    pub(crate) date: String,
    pub(crate) name: Option<String>,
    pub(crate) address: Address,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Load {
    pub(crate) id: String,
    pub(crate) shipper_id: String,
    pub(crate) actor_id: String,
    pub(crate) created_at: Instant,
    pub(crate) stops: Vec<Stop>,
    events: Vec<LoadEvent>,
}

impl Load {
    #[allow(dead_code)]
    pub(crate) fn reconstitute(
        id: String,
        shipper_id: String,
        actor_id: String,
        created_at: Instant,
        stops: Vec<Stop>,
    ) -> Self {
        Self {
            id,
            shipper_id,
            actor_id,
            created_at,
            stops,
            events: Vec::new(),
        }
    }

    pub(crate) fn create(
        id: String,
        shipper_id: String,
        actor_id: String,
        created_at: Instant,
        pickup: Stop,
        delivery: Stop,
    ) -> Result<Self, LoadError> {
        if shipper_id.trim().is_empty() {
            return Err(LoadError::EmptyShipperId);
        }
        if actor_id.trim().is_empty() {
            return Err(LoadError::EmptyActorId);
        }
        if pickup.kind != StopKind::Pickup || delivery.kind != StopKind::Delivery {
            return Err(LoadError::StopKinds);
        }
        match &delivery.name {
            Some(name) if !name.trim().is_empty() => {}
            _ => return Err(LoadError::MissingConsignee),
        }
        if delivery.date < pickup.date {
            return Err(LoadError::DeliveryBeforePickup);
        }
        Ok(Self {
            id: id.clone(),
            shipper_id,
            actor_id,
            created_at,
            stops: vec![pickup, delivery],
            events: vec![LoadEvent::Created { load_id: id }],
        })
    }

    pub(crate) fn pull_events(&mut self) -> Vec<LoadEvent> {
        std::mem::take(&mut self.events)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum LoadError {
    EmptyShipperId,
    EmptyActorId,
    StopKinds,
    MissingConsignee,
    DeliveryBeforePickup,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct LoadRow {
    pub(crate) id: String,
    pub(crate) shipper_id: String,
    pub(crate) actor_id: String,
    pub(crate) created_at: Instant,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct StopRow {
    pub(crate) id: String,
    pub(crate) load_id: String,
    pub(crate) kind: String,
    pub(crate) date: String,
    pub(crate) name: Option<String>,
    pub(crate) line1: String,
    pub(crate) line2: Option<String>,
    pub(crate) city: String,
    pub(crate) region: String,
    pub(crate) postal_code: String,
    pub(crate) country: String,
}

pub(crate) fn to_rows(load: &Load) -> (LoadRow, Vec<StopRow>) {
    let load_row = LoadRow {
        id: load.id.clone(),
        shipper_id: load.shipper_id.clone(),
        actor_id: load.actor_id.clone(),
        created_at: load.created_at,
    };
    let stop_rows = load
        .stops
        .iter()
        .map(|stop| StopRow {
            id: stop.id.clone(),
            load_id: load.id.clone(),
            kind: stop.kind.as_str().to_owned(),
            date: stop.date.clone(),
            name: stop.name.clone(),
            line1: stop.address.line1.clone(),
            line2: stop.address.line2.clone(),
            city: stop.address.city.clone(),
            region: stop.address.region.clone(),
            postal_code: stop.address.postal_code.clone(),
            country: stop.address.country.clone(),
        })
        .collect();
    (load_row, stop_rows)
}

#[allow(dead_code)]
pub(crate) fn from_rows(load: LoadRow, stops: Vec<StopRow>) -> Load {
    let mut stops: Vec<Stop> = stops
        .into_iter()
        .map(|stop| Stop {
            id: stop.id,
            kind: StopKind::parse(&stop.kind),
            date: stop.date,
            name: stop.name,
            address: Address {
                line1: stop.line1,
                line2: stop.line2,
                city: stop.city,
                region: stop.region,
                postal_code: stop.postal_code,
                country: stop.country,
            },
        })
        .collect();
    stops.sort_by_key(|stop| stop.kind);
    Load::reconstitute(
        load.id,
        load.shipper_id,
        load.actor_id,
        load.created_at,
        stops,
    )
}

#[cfg(test)]
pub(crate) struct LoadBuilder {
    id: String,
    shipper_id: String,
    actor_id: String,
    created_at: Instant,
    pickup_id: String,
    pickup_date: String,
    pickup_name: Option<String>,
    pickup_line2: Option<String>,
    delivery_id: String,
    delivery_date: String,
    delivery_name: String,
    delivery_line2: Option<String>,
}

#[cfg(test)]
impl LoadBuilder {
    pub(crate) fn new() -> Self {
        Self {
            id: "01900000-0000-7000-8000-000000000001".to_owned(),
            shipper_id: "shipper-1".to_owned(),
            actor_id: "actor-1".to_owned(),
            created_at: Instant::from_unix_timestamp(1_700_000_000).unwrap(),
            pickup_id: "01900000-0000-7000-8000-000000000002".to_owned(),
            pickup_date: "2026-09-20".to_owned(),
            pickup_name: None,
            pickup_line2: None,
            delivery_id: "01900000-0000-7000-8000-000000000003".to_owned(),
            delivery_date: "2026-09-21".to_owned(),
            delivery_name: "Consignee".to_owned(),
            delivery_line2: None,
        }
    }

    pub(crate) fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub(crate) fn pickup_id(mut self, id: impl Into<String>) -> Self {
        self.pickup_id = id.into();
        self
    }

    pub(crate) fn delivery_id(mut self, id: impl Into<String>) -> Self {
        self.delivery_id = id.into();
        self
    }

    pub(crate) fn pickup_name(mut self, name: impl Into<String>) -> Self {
        self.pickup_name = Some(name.into());
        self
    }

    pub(crate) fn pickup_line2(mut self, line2: impl Into<String>) -> Self {
        self.pickup_line2 = Some(line2.into());
        self
    }

    pub(crate) fn shipper_id(mut self, shipper_id: impl Into<String>) -> Self {
        self.shipper_id = shipper_id.into();
        self
    }

    pub(crate) fn actor_id(mut self, actor_id: impl Into<String>) -> Self {
        self.actor_id = actor_id.into();
        self
    }

    pub(crate) fn delivery_date(mut self, date: impl Into<String>) -> Self {
        self.delivery_date = date.into();
        self
    }

    pub(crate) fn build(self) -> Load {
        let (id, shipper_id, actor_id, created_at, pickup, delivery) = self.into_parts();
        Load::reconstitute(id, shipper_id, actor_id, created_at, vec![pickup, delivery])
    }

    pub(crate) fn build_new(self) -> Load {
        let (id, shipper_id, actor_id, created_at, pickup, delivery) = self.into_parts();
        Load::create(id, shipper_id, actor_id, created_at, pickup, delivery)
            .expect("builder fixture is valid")
    }

    fn into_parts(self) -> (String, String, String, Instant, Stop, Stop) {
        (
            self.id,
            self.shipper_id,
            self.actor_id,
            self.created_at,
            Stop {
                id: self.pickup_id,
                kind: StopKind::Pickup,
                date: self.pickup_date,
                name: self.pickup_name,
                address: Address {
                    line1: "1 Dock".to_owned(),
                    line2: self.pickup_line2,
                    city: "Dallas".to_owned(),
                    region: "TX".to_owned(),
                    postal_code: "75201".to_owned(),
                    country: "US".to_owned(),
                },
            },
            Stop {
                id: self.delivery_id,
                kind: StopKind::Delivery,
                date: self.delivery_date,
                name: Some(self.delivery_name),
                address: Address {
                    line1: "9 Warehouse".to_owned(),
                    line2: self.delivery_line2,
                    city: "Austin".to_owned(),
                    region: "TX".to_owned(),
                    postal_code: "78701".to_owned(),
                    country: "US".to_owned(),
                },
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Load, LoadBuilder, LoadError, from_rows, to_rows};
    use crate::domain::events::LoadEvent;

    #[test]
    fn mapper_round_trips_a_load_without_optionals() {
        let load = LoadBuilder::new().build();
        let (row, stops) = to_rows(&load);
        assert_eq!(from_rows(row, stops), load);
    }

    #[test]
    fn mapper_round_trips_pickup_name_and_line2() {
        let load = LoadBuilder::new()
            .pickup_name("Yard")
            .pickup_line2("Suite 4")
            .build();
        let (row, stops) = to_rows(&load);
        assert_eq!(from_rows(row, stops), load);
    }

    #[test]
    fn mapper_orders_pickup_before_delivery() {
        let load = LoadBuilder::new().build();
        let (row, mut stops) = to_rows(&load);
        stops.reverse();
        let restored = from_rows(row, stops);
        assert_eq!(restored.stops[0].kind, super::StopKind::Pickup);
        assert_eq!(restored.stops[1].kind, super::StopKind::Delivery);
    }

    #[test]
    fn create_accepts_two_typed_stops() {
        let load = LoadBuilder::new().build_new();
        assert_eq!(load.shipper_id, "shipper-1");
        assert_eq!(load.stops[0].kind, super::StopKind::Pickup);
        assert_eq!(load.stops[1].kind, super::StopKind::Delivery);
    }

    #[test]
    fn create_records_load_created() {
        let mut load = LoadBuilder::new().build_new();
        let id = load.id.clone();
        assert_eq!(load.pull_events(), vec![LoadEvent::Created { load_id: id }]);
        assert_eq!(load.pull_events(), vec![]);
    }

    #[test]
    fn reconstitute_records_no_events() {
        let mut load = LoadBuilder::new().build();
        assert_eq!(load.pull_events(), vec![]);
    }

    #[test]
    fn create_rejects_empty_shipper_id() {
        let (id, shipper_id, actor_id, created_at, pickup, delivery) =
            LoadBuilder::new().shipper_id("  ").into_parts();
        assert_eq!(
            Load::create(id, shipper_id, actor_id, created_at, pickup, delivery),
            Err(LoadError::EmptyShipperId)
        );
    }

    #[test]
    fn create_rejects_empty_actor_id() {
        let (id, shipper_id, actor_id, created_at, pickup, delivery) =
            LoadBuilder::new().actor_id("  ").into_parts();
        assert_eq!(
            Load::create(id, shipper_id, actor_id, created_at, pickup, delivery),
            Err(LoadError::EmptyActorId)
        );
    }

    #[test]
    fn create_rejects_stop_kinds() {
        let (id, shipper_id, actor_id, created_at, mut pickup, mut delivery) =
            LoadBuilder::new().into_parts();
        pickup.kind = super::StopKind::Delivery;
        delivery.kind = super::StopKind::Pickup;
        assert_eq!(
            Load::create(id, shipper_id, actor_id, created_at, pickup, delivery),
            Err(LoadError::StopKinds)
        );
    }

    #[test]
    fn create_rejects_missing_consignee() {
        let (id, shipper_id, actor_id, created_at, pickup, mut delivery) =
            LoadBuilder::new().into_parts();
        delivery.name = None;
        assert_eq!(
            Load::create(id, shipper_id, actor_id, created_at, pickup, delivery),
            Err(LoadError::MissingConsignee)
        );
    }

    #[test]
    fn create_rejects_delivery_before_pickup() {
        let (id, shipper_id, actor_id, created_at, pickup, delivery) =
            LoadBuilder::new().delivery_date("2026-09-19").into_parts();
        assert_eq!(
            Load::create(id, shipper_id, actor_id, created_at, pickup, delivery),
            Err(LoadError::DeliveryBeforePickup)
        );
    }
}
