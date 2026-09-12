use crate::Loads;
use axum::Router;
use axum::routing::post;
use kernel::{Clock, Logger, Metrics};

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
