use super::{ActorId, CorrelationId};
use crate::Loads;
use crate::domain::api::create_load::{
    CreateLoad, CreateLoadError, CreateLoadInput, LoadResource, ViolationDto,
};
use axum::Json;
use axum::body::Bytes;
use axum::extract::{Extension, OriginalUri, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use kernel::{Clock, Logger, Metrics};
use serde::Serialize;
use serde_json::error::Category;
use utoipa::ToSchema;

const PROBLEM_JSON: &str = "application/problem+json";
const IDEMPOTENCY_KEY: &str = "idempotency-key";

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Problem {
    r#type: String,
    status: u16,
    detail: String,
    instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    violations: Option<Vec<ViolationDto>>,
}

#[utoipa::path(
    post,
    path = "/loads",
    tag = "loads",
    operation_id = "create_load",
    request_body = CreateLoadInput,
    params(("Idempotency-Key" = String, Header)),
    responses(
        (status = 201, description = "Load created", body = LoadResource),
        (
            status = 400,
            description = "VALIDATION_FAILED",
            body = Problem,
            content_type = "application/problem+json"
        ),
        (
            status = 409,
            description = "LOAD_CONFLICT",
            body = Problem,
            content_type = "application/problem+json"
        )
    )
)]
pub(crate) async fn create_load<C, L, M>(
    State(cell): State<Loads<C, L, M>>,
    Extension(ActorId(actor)): Extension<ActorId>,
    correlation: Option<Extension<CorrelationId>>,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    body: Bytes,
) -> Response
where
    C: Clock + Clone + Send + Sync + 'static,
    L: Logger + Clone + Send + Sync + 'static,
    M: Metrics + Clone + Send + Sync + 'static,
{
    let instance = uri.path();
    let correlation = correlation.as_ref().map(|Extension(id)| id.0.as_str());
    let idempotency_key = match read_idempotency_key(&headers) {
        Ok(key) => key,
        Err(error) => return problem(error, instance, correlation),
    };
    let input = match deserialize_body(&body) {
        Ok(input) => input,
        Err(error) => return problem(error, instance, correlation),
    };
    match cell
        .create_load
        .create_load(actor, idempotency_key, input)
        .await
    {
        Ok(resource) => (StatusCode::CREATED, Json(resource)).into_response(),
        Err(error) => problem(error, instance, correlation),
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
    serde_path_to_error::deserialize(&mut de).map_err(|error| {
        let message = match error.inner().classify() {
            Category::Data => "invalid value",
            Category::Syntax | Category::Eof | Category::Io => "invalid json",
        };
        CreateLoadError::ValidationFailed {
            violations: vec![ViolationDto {
                path: error.path().to_string(),
                code: "invalid".to_owned(),
                message: message.to_owned(),
            }],
        }
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
    let (status, r#type, detail, violations) = match error {
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
        CreateLoadError::LoadInvalid => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "LOAD_INVALID",
            "The Load could not be created.",
            None,
        ),
    };
    let body = Problem {
        r#type: r#type.to_owned(),
        status: status.as_u16(),
        detail: detail.to_owned(),
        instance: instance.to_owned(),
        correlation_id: correlation_id
            .filter(|id| !id.is_empty())
            .map(str::to_owned),
        violations,
    };
    (
        status,
        [(header::CONTENT_TYPE, PROBLEM_JSON)],
        serde_json::to_string(&body).expect("problem serializes"),
    )
        .into_response()
}
