use crate::Loads;
use axum::Router;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use kernel::{Clock, Logger, Metrics};

const PROBLEM_JSON: &str = "application/problem+json";

// ponytail: process-wide until a second cell extracts
#[derive(Clone)]
pub struct ActorId(pub String);

#[derive(Clone)]
pub struct CorrelationId(pub String);

impl<S> FromRequestParts<S> for ActorId
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.extensions.get::<ActorId>() {
            Some(actor) => Ok(actor.clone()),
            None => {
                let instance = parts.uri.path().to_owned();
                let correlation = parts
                    .extensions
                    .get::<CorrelationId>()
                    .map(|id| id.0.clone());
                Err(unauthenticated(&instance, correlation.as_deref()))
            }
        }
    }
}

fn unauthenticated(instance: &str, correlation_id: Option<&str>) -> Response {
    let mut body = serde_json::json!({
        "type": "UNAUTHENTICATED",
        "status": 401,
        "detail": "Authentication is required.",
        "instance": instance,
    });
    if let Some(id) = correlation_id.filter(|id| !id.is_empty()) {
        body["correlationId"] = serde_json::Value::String(id.to_owned());
    }
    (
        StatusCode::UNAUTHORIZED,
        [(header::CONTENT_TYPE, PROBLEM_JSON)],
        body.to_string(),
    )
        .into_response()
}

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
