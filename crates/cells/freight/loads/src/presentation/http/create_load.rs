use crate::Loads;
use crate::application::commands::create_load::CreateLoadFailure;
use crate::domain::api::create_load::{CreateLoadError, CreateLoadInput, ViolationDto};
use axum::Json;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use kernel::{Clock, Logger, Metrics};

const PROBLEM_JSON: &str = "application/problem+json";
const IDEMPOTENCY_KEY: &str = "idempotency-key";
const ACTOR_ID: &str = "x-actor-id";
const CORRELATION_ID: &str = "x-correlation-id";

pub(crate) async fn create_load<C, L, M>(
    State(cell): State<Loads<C, L, M>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    let instance = "/loads";
    let correlation = header_str(&headers, CORRELATION_ID);
    let idempotency_key = match read_idempotency_key(&headers) {
        Ok(key) => key,
        Err(error) => return problem(error, instance, correlation),
    };
    let actor_id = match header_str(&headers, ACTOR_ID).filter(|id| !id.is_empty()) {
        Some(id) => id.to_owned(),
        None => {
            return unhandled(instance, correlation);
        }
    };
    let input = match deserialize_body(&body) {
        Ok(input) => input,
        Err(error) => return problem(error, instance, correlation),
    };
    match cell.create_load(actor_id, idempotency_key, input).await {
        Ok(resource) => (StatusCode::CREATED, Json(resource)).into_response(),
        Err(CreateLoadFailure::Envelope(error)) => problem(error, instance, correlation),
        Err(CreateLoadFailure::Unexpected) => unhandled(instance, correlation),
    }
}

fn read_idempotency_key(headers: &HeaderMap) -> Result<String, CreateLoadError> {
    let Some(value) = headers.get(IDEMPOTENCY_KEY) else {
        return Err(header_violation("required", "is required"));
    };
    let key = value.to_str().unwrap_or("");
    if key.is_empty() {
        return Err(header_violation("empty", "must not be empty"));
    }
    if key.len() > 255 {
        return Err(header_violation("length", "must be at most 255 characters"));
    }
    Ok(key.to_owned())
}

fn deserialize_body(body: &Bytes) -> Result<CreateLoadInput, CreateLoadError> {
    let mut de = serde_json::Deserializer::from_slice(body);
    serde_path_to_error::deserialize(&mut de).map_err(|error| CreateLoadError::ValidationFailed {
        violations: vec![ViolationDto {
            path: error.path().to_string(),
            code: "invalid".to_owned(),
            message: error.inner().to_string(),
        }],
    })
}

fn header_violation(code: &str, message: &str) -> CreateLoadError {
    CreateLoadError::ValidationFailed {
        violations: vec![ViolationDto {
            path: "Idempotency-Key".to_owned(),
            code: code.to_owned(),
            message: message.to_owned(),
        }],
    }
}

fn problem(error: CreateLoadError, instance: &str, correlation_id: Option<&str>) -> Response {
    let (status, r#type, detail, violations) = match &error {
        CreateLoadError::ValidationFailed { violations } => (
            StatusCode::BAD_REQUEST,
            "VALIDATION_FAILED",
            "Request validation failed.",
            Some(violations),
        ),
        CreateLoadError::LoadConflict => (
            StatusCode::CONFLICT,
            "LOAD_CONFLICT",
            "The request conflicts with an existing Load.",
            None,
        ),
    };
    let mut body = serde_json::json!({
        "type": r#type,
        "status": status.as_u16(),
        "detail": detail,
        "instance": instance,
    });
    if let Some(id) = correlation_id.filter(|id| !id.is_empty()) {
        body["correlationId"] = serde_json::Value::String(id.to_owned());
    }
    if let Some(violations) = violations {
        body["violations"] = serde_json::to_value(violations).expect("violations serialize");
    }
    (
        status,
        [(header::CONTENT_TYPE, PROBLEM_JSON)],
        body.to_string(),
    )
        .into_response()
}

fn unhandled(instance: &str, correlation_id: Option<&str>) -> Response {
    let mut body = serde_json::json!({
        "type": "about:blank",
        "status": 500,
        "detail": "Internal server error",
        "instance": instance,
    });
    if let Some(id) = correlation_id.filter(|id| !id.is_empty()) {
        body["correlationId"] = serde_json::Value::String(id.to_owned());
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, PROBLEM_JSON)],
        body.to_string(),
    )
        .into_response()
}

fn header_str<'a>(headers: &'a HeaderMap, name: &'static str) -> Option<&'a str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}
