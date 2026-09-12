mod application;
mod domain;
mod infrastructure;
mod presentation;

pub use crate::infrastructure::LoadsPool;
pub use crate::presentation::http::{ActorId, CorrelationId};

use crate::infrastructure::load_store::SqlxLoadStore;
use kernel::{Clock, Logger, Metrics};

#[derive(Clone)]
pub struct Loads<C, L, M> {
    pub(crate) store: SqlxLoadStore,
    pub(crate) clock: C,
    pub(crate) logger: L,
    #[allow(dead_code)]
    pub(crate) metrics: M,
}

pub fn new<C, L, M>(pool: LoadsPool, clock: C, logger: L, metrics: M) -> Loads<C, L, M>
where
    C: Clock,
    L: Logger,
    M: Metrics,
{
    Loads {
        store: SqlxLoadStore::new(pool),
        clock,
        logger,
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
