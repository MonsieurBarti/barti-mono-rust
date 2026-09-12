use crate::Loads;
use kernel::{Clock, Logger, Metrics};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

// ponytail: process-wide until a second cell extracts
#[derive(Clone)]
pub struct ActorId(pub String);

#[derive(Clone)]
pub struct CorrelationId(pub String);

pub(crate) fn router<C, L, M>(cell: &Loads<C, L, M>) -> OpenApiRouter
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    routes::<C, L, M>().with_state(cell.clone())
}

pub(crate) fn routes<C, L, M>() -> OpenApiRouter<Loads<C, L, M>>
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    OpenApiRouter::new().routes(routes!(create_load::create_load))
}

mod create_load;
