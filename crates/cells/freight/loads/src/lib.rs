mod application;
mod domain;
mod infrastructure;
mod migrations;
mod presentation;

pub use crate::infrastructure::LoadsPool;
pub use crate::migrations::{Migrator, SCHEMA};
pub use crate::presentation::http::{ActorId, CorrelationId};

use crate::application::commands::create_load::CreateLoadCommand;
use crate::infrastructure::load_events::InCellLoadEvents;
use crate::infrastructure::load_store::SeaOrmLoadStore;
use kernel::{Clock, Logger, Metrics, SystemClock};
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(Clone)]
pub struct Loads<C, L, M> {
    pub(crate) create_load: CreateLoadCommand<SeaOrmLoadStore, InCellLoadEvents, C, L>,
    #[allow(dead_code)]
    pub(crate) metrics: M,
}

pub fn new<C: Clock, L: Logger, M: Metrics>(
    pool: LoadsPool,
    clock: C,
    logger: L,
    metrics: M,
) -> Loads<C, L, M> {
    Loads {
        create_load: CreateLoadCommand {
            store: SeaOrmLoadStore::new(pool),
            events: InCellLoadEvents,
            clock,
            logger,
        },
        metrics,
    }
}

pub fn router<C, L, M>(cell: &Loads<C, L, M>) -> OpenApiRouter
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    presentation::http::router(cell)
}

pub fn openapi() -> OpenApi {
    // ponytail: type params only; a pool is not required to collect routes!
    #[derive(Clone, Copy)]
    struct Silent;
    impl Logger for Silent {
        fn debug(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn info(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn warn(&self, _msg: &str, _fields: &[(&str, &str)]) {}
        fn error(&self, _msg: &str, _fields: &[(&str, &str)]) {}
    }
    impl Metrics for Silent {
        fn increment(&self, _name: &str, _value: u64, _tags: &[(&str, &str)]) {}
        fn distribution(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}
    }
    presentation::http::routes::<SystemClock, Silent, Silent>().into_openapi()
}
