use axum::Router;
use axum::body::Body;
use axum::extract::Request;
use axum::http::header;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use std::time::Instant;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::Instrument;
use uuid::{Uuid, Version};

const X_CORRELATION_ID: HeaderName = HeaderName::from_static("x-correlation-id");
const X_ACTOR_ID: HeaderName = HeaderName::from_static("x-actor-id");
const PROBLEM_JSON: &str = "application/problem+json";
const SLOW_MS: u64 = 3000;

#[derive(Clone)]
struct CorrelationId(String);

// ponytail: read only by the test probe until ticket 08 lands `loads::router`.
#[derive(Clone)]
#[cfg_attr(not(test), allow(dead_code))]
struct ActorId(String);

// `loads` exports no `router` yet (ticket 08). Nest it here when it does.
pub(crate) fn router() -> Router {
    layered(Router::new().route("/health", get(health)))
}

fn layered(router: Router) -> Router {
    router
        .layer(CatchPanicLayer::custom(panic_response))
        .layer(middleware::from_fn(wide_event))
        .layer(middleware::from_fn(identity))
        .layer(middleware::from_fn(correlation))
}

#[cfg(test)]
fn test_router() -> Router {
    layered(
        Router::new()
            .route("/health", get(health))
            .route("/_probe", get(probe_actor))
            .route("/_panic", get(panic_probe)),
    )
}

#[cfg(test)]
async fn probe_actor(axum::Extension(ActorId(id)): axum::Extension<ActorId>) -> String {
    id
}

#[cfg(test)]
async fn panic_probe() {
    panic!("boom");
}

async fn health() {}

async fn correlation(mut request: Request, next: Next) -> Response {
    let id = resolve_correlation_id(&request);
    request.extensions_mut().insert(CorrelationId(id.clone()));
    let span = tracing::info_span!("http.request", correlationId = %id);
    let mut response = next.run(request).instrument(span).await;
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
        .and_then(|value| {
            Uuid::parse_str(value)
                .ok()
                .filter(|id| matches!(id.get_version(), Some(Version::Random | Version::SortRand)))
                .map(|_| value.to_owned())
        })
        .unwrap_or_else(|| Uuid::now_v7().to_string())
}

async fn identity(mut request: Request, next: Next) -> Response {
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }
    match request
        .headers()
        .get(&X_ACTOR_ID)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
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
        .filter(|id| !id.is_empty());
    problem(
        StatusCode::UNAUTHORIZED,
        "UNAUTHENTICATED",
        "Authentication is required.",
        request.uri().path(),
        correlation_id,
    )
}

fn problem_json(
    status: StatusCode,
    r#type: &str,
    detail: &str,
    instance: &str,
    correlation_id: Option<&str>,
) -> String {
    let mut body = serde_json::json!({
        "type": r#type,
        "status": status.as_u16(),
        "detail": detail,
        "instance": instance,
    });
    if let Some(id) = correlation_id.filter(|id| !id.is_empty()) {
        body["correlationId"] = serde_json::Value::String(id.to_owned());
    }
    body.to_string()
}

fn problem(
    status: StatusCode,
    r#type: &str,
    detail: &str,
    instance: &str,
    correlation_id: Option<&str>,
) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, PROBLEM_JSON)],
        problem_json(status, r#type, detail, instance, correlation_id),
    )
        .into_response()
}

fn panic_response(_: Box<dyn std::any::Any + Send + 'static>) -> axum::http::Response<String> {
    axum::http::Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, PROBLEM_JSON)
        .body(problem_json(
            StatusCode::INTERNAL_SERVER_ERROR,
            "about:blank",
            "Internal server error",
            "",
            None,
        ))
        .expect("static panic response")
}

async fn wide_event(request: Request, next: Next) -> Response {
    if request.uri().path() == "/health" {
        return next.run(request).await;
    }
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let correlation_id = request
        .extensions()
        .get::<CorrelationId>()
        .map(|id| id.0.clone())
        .unwrap_or_default();
    let actor_id = request.extensions().get::<ActorId>().map(|id| id.0.clone());
    let started = Instant::now();
    let response = next.run(request).await;
    let duration_ms = started.elapsed().as_millis() as u64;
    let status = response.status();
    let (response, envelope_type) = problem_envelope_type(response).await;
    emit_wide_event(
        status,
        duration_ms,
        &correlation_id,
        actor_id.as_deref(),
        &method,
        &path,
        envelope_type.as_deref(),
    );
    response
}

fn emit_wide_event(
    status: StatusCode,
    duration_ms: u64,
    correlation_id: &str,
    actor_id: Option<&str>,
    method: &axum::http::Method,
    path: &str,
    envelope_type: Option<&str>,
) {
    let actor_id = actor_id.unwrap_or("");
    let envelope_type = envelope_type.unwrap_or("");
    let status = status.as_u16();
    if status >= 500 {
        tracing::error!(
            msg = "app.http.request",
            correlationId = correlation_id,
            actorId = actor_id,
            method = %method,
            path,
            duration_ms,
            status,
            r#type = envelope_type,
            source = "api",
        );
    } else if status >= 400 {
        tracing::warn!(
            msg = "app.http.request",
            correlationId = correlation_id,
            actorId = actor_id,
            method = %method,
            path,
            duration_ms,
            status,
            r#type = envelope_type,
            source = "api",
        );
    } else if duration_ms > SLOW_MS {
        tracing::info!(
            msg = "app.http.request",
            correlationId = correlation_id,
            actorId = actor_id,
            method = %method,
            path,
            duration_ms,
            status,
            r#type = envelope_type,
            source = "api",
        );
    } else {
        tracing::debug!(
            msg = "app.http.request",
            correlationId = correlation_id,
            actorId = actor_id,
            method = %method,
            path,
            duration_ms,
            status,
            r#type = envelope_type,
            source = "api",
        );
    }
}

async fn problem_envelope_type(response: Response) -> (Response, Option<String>) {
    let is_problem = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        == Some(PROBLEM_JSON);
    if !is_problem {
        return (response, None);
    }
    let (parts, body) = response.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 16 * 1024).await else {
        return (Response::from_parts(parts, Body::empty()), None);
    };
    let envelope_type = serde_json::from_slice::<serde_json::Value>(&bytes)
        .ok()
        .and_then(|value| value.get("type")?.as_str().map(str::to_owned));
    (
        Response::from_parts(parts, Body::from(bytes)),
        envelope_type,
    )
}

#[cfg(test)]
mod tests {
    use super::test_router;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::ServiceExt;
    use uuid::{Uuid, Version};

    async fn health(correlation: Option<&str>) -> axum::http::Response<Body> {
        let mut request = Request::builder().uri("/health");
        if let Some(value) = correlation {
            request = request.header("X-Correlation-ID", value);
        }
        test_router()
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
    async fn head_health_succeeds_without_actor_id() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .method(Method::HEAD)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
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
    async fn health_echoes_uppercase_v4_correlation_id_verbatim() {
        let id = "550E8400-E29B-41D4-A716-446655440000";
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

    #[tokio::test]
    async fn health_mints_uuid_v7_when_correlation_id_is_v1() {
        let response = health(Some("6ba7b810-9dad-11d1-80b4-00c04fd430c8")).await;
        assert_eq!(response.status(), StatusCode::OK);
        let parsed = Uuid::parse_str(correlation_id(&response)).unwrap();
        assert_eq!(parsed.get_version(), Some(Version::SortRand));
    }

    async fn get(path: &str, actor: Option<&str>) -> axum::http::Response<Body> {
        let mut request = Request::builder().uri(path);
        if let Some(actor) = actor {
            request = request.header("X-Actor-Id", actor);
        }
        test_router()
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
    async fn human_rest_with_actor_id_is_not_found() {
        let response = get("/loads", Some("actor-1")).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn empty_actor_id_is_unauthenticated() {
        let response = get("/loads", Some("")).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn whitespace_actor_id_is_unauthenticated() {
        let response = get("/loads", Some("   ")).await;
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

    #[tokio::test]
    async fn panic_is_about_blank_problem() {
        let response = get("/_panic", Some("actor-1")).await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .unwrap()
                .to_str()
                .unwrap(),
            "application/problem+json"
        );
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["status"], 500);
        assert_eq!(body["detail"], "Internal server error");
        assert!(body.get("title").is_none());
        assert!(body.get("context").is_none());
    }
}
