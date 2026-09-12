use crate::Loads;
use axum::Router;
use axum::routing::post;
use kernel::{Clock, Logger, Metrics};

// ponytail: process-wide until a second cell extracts
#[derive(Clone)]
pub struct ActorId(pub String);

#[derive(Clone)]
pub struct CorrelationId(pub String);

pub(crate) fn router<C, L, M>(cell: &Loads<C, L, M>) -> Router
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/loads", post(create_load::create_load::<C, L, M>))
        .with_state(cell.clone())
}

mod create_load;
