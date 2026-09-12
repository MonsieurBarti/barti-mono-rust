use axum::Router;
use axum::extract::Request;
use axum::http::header;
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use uuid::{Uuid, Version};

const X_CORRELATION_ID: HeaderName = HeaderName::from_static("x-correlation-id");
const X_ACTOR_ID: HeaderName = HeaderName::from_static("x-actor-id");

#[derive(Clone)]
struct CorrelationId(String);

// ponytail: read only by the test probe until ticket 08 lands `loads::router`.
#[derive(Clone)]
#[cfg_attr(not(test), allow(dead_code))]
struct ActorId(String);

// `loads` exports no `router` yet (ticket 08). Nest it here when it does.
pub fn router() -> Router {
    let router = Router::new().route("/health", get(health));
    #[cfg(test)]
    let router = router.route("/_probe", get(probe_actor));
    router
        .layer(middleware::from_fn(identity))
        .layer(middleware::from_fn(correlation))
}

#[cfg(test)]
async fn probe_actor(axum::Extension(ActorId(id)): axum::Extension<ActorId>) -> String {
    id
}

async fn health() {}

async fn correlation(mut request: Request, next: Next) -> Response {
    let id = resolve_correlation_id(&request);
    request.extensions_mut().insert(CorrelationId(id.clone()));
    let mut response = next.run(request).await;
    if let Ok(value) = HeaderValue::from_str(&id) {
        response.headers_mut().insert(X_CORRELATION_ID, value);
    }
    response
}

fn resolve_correlation_id(request: &Request) -> String {
    request
        .headers()
        .get(&X_CORRELATION_ID)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| Uuid::parse_str(value).ok())
        .filter(|id| matches!(id.get_version(), Some(Version::Random | Version::SortRand)))
        .map(|id| id.to_string())
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

async fn identity(mut request: Request, next: Next) -> Response {
    if request.method() == Method::GET && request.uri().path() == "/health" {
        return next.run(request).await;
    }
    match request
        .headers()
        .get(&X_ACTOR_ID)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
    {
        Some(actor) => {
            request.extensions_mut().insert(ActorId(actor));
            next.run(request).await
        }
        None => unauthenticated(&request),
    }
}

fn unauthenticated(request: &Request) -> Response {
    let correlation_id = request
        .extensions()
        .get::<CorrelationId>()
        .map(|id| id.0.as_str())
        .unwrap_or_default();
    let body = serde_json::json!({
        "type": "UNAUTHENTICATED",
        "status": 401,
        "detail": "Authentication is required.",
        "instance": request.uri().path(),
        "correlationId": correlation_id,
    });
    (
        StatusCode::UNAUTHORIZED,
        [(header::CONTENT_TYPE, "application/problem+json")],
        body.to_string(),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use uuid::{Uuid, Version};

    async fn health(correlation: Option<&str>) -> axum::http::Response<Body> {
        let mut request = Request::builder().uri("/health");
        if let Some(value) = correlation {
            request = request.header("X-Correlation-ID", value);
        }
        router()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    fn correlation_id(response: &axum::http::Response<Body>) -> &str {
        response
            .headers()
            .get("X-Correlation-ID")
            .unwrap()
            .to_str()
            .unwrap()
    }

    #[tokio::test]
    async fn health_succeeds_without_actor_id() {
        let response = health(None).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn health_echoes_provided_correlation_id() {
        let id = "550e8400-e29b-41d4-a716-446655440000";
        let response = health(Some(id)).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(correlation_id(&response), id);
    }

    #[tokio::test]
    async fn health_mints_uuid_v7_when_correlation_id_is_missing() {
        let response = health(None).await;
        assert_eq!(response.status(), StatusCode::OK);
        let parsed = Uuid::parse_str(correlation_id(&response)).unwrap();
        assert_eq!(parsed.get_version(), Some(Version::SortRand));
    }

    #[tokio::test]
    async fn health_mints_uuid_v7_when_correlation_id_is_invalid() {
        let response = health(Some("not-a-uuid")).await;
        assert_eq!(response.status(), StatusCode::OK);
        let parsed = Uuid::parse_str(correlation_id(&response)).unwrap();
        assert_eq!(parsed.get_version(), Some(Version::SortRand));
    }

    async fn get(path: &str, actor: Option<&str>) -> axum::http::Response<Body> {
        let mut request = Request::builder().uri(path);
        if let Some(actor) = actor {
            request = request.header("X-Actor-Id", actor);
        }
        router()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn human_rest_without_actor_id_is_unauthenticated() {
        let response = get("/loads", None).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .unwrap()
                .to_str()
                .unwrap(),
            "application/problem+json"
        );
        let correlation = correlation_id(&response).to_string();
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["type"], "UNAUTHENTICATED");
        assert_eq!(body["status"], 401);
        assert_eq!(body["instance"], "/loads");
        assert_eq!(body["correlationId"], correlation);
        assert!(body.get("title").is_none());
        assert!(body.get("context").is_none());
    }

    #[tokio::test]
    async fn human_rest_with_actor_id_is_not_unauthenticated() {
        let response = get("/loads", Some("actor-1")).await;
        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn empty_actor_id_is_unauthenticated() {
        let response = get("/loads", Some("")).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn copies_actor_id_into_extensions() {
        let response = get("/_probe", Some("actor-1")).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(&body[..], b"actor-1");
    }
}
