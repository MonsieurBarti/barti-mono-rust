#![allow(dead_code)]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::LoadsPool;
use crate::infrastructure::load_store::SqlxLoadStore;
use kernel::{Clock, Logger, Metrics};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Loads<C, L, M> {
    store: SqlxLoadStore,
    clock: C,
    logger: L,
    #[allow(dead_code)]
    metrics: M,
}

pub fn new<C, L, M>(pool: PgPool, clock: C, logger: L, metrics: M) -> Loads<C, L, M>
where
    C: Clock,
    L: Logger,
    M: Metrics,
{
    Loads {
        store: SqlxLoadStore::new(LoadsPool::new(pool)),
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

impl<C, L, M> Loads<C, L, M>
where
    C: Clock + Send + Sync,
    L: Logger + Send + Sync,
    M: Metrics + Send + Sync,
{
    pub(crate) async fn create_load(
        &self,
        actor_id: String,
        idempotency_key: String,
        input: crate::domain::api::create_load::CreateLoadInput,
    ) -> Result<
        crate::domain::api::create_load::LoadResource,
        crate::application::commands::create_load::CreateLoadFailure,
    > {
        let result = application::commands::create_load::run(
            &self.store,
            &self.clock,
            actor_id,
            idempotency_key,
            input,
        )
        .await;
        if matches!(
            result,
            Err(crate::application::commands::create_load::CreateLoadFailure::Unexpected)
        ) {
            self.logger
                .error("loads.create_load.failed", &[("error", "unexpected")]);
        }
        result
    }
}
