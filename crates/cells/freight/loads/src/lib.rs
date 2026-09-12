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
use kernel::{Clock, Logger, Metrics};

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

pub fn router<C, L, M>(cell: &Loads<C, L, M>) -> axum::Router
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    presentation::http::router(cell)
}
