use axum::body::Body;
use axum::http::{Request, StatusCode};
use kernel::{FakeClock, Instant, Logger, Metrics};
use sea_orm::{ConnectOptions, ConnectionTrait, Database};
use sea_orm_migration::MigratorTrait;
use tower::ServiceExt;
use uuid::Uuid;

#[derive(Clone)]
struct Silent;

impl Logger for Silent {
    fn debug(&self, _msg: &str, _fields: &[(&str, &str)]) {}
    fn info(&self, _msg: &str, _fields: &[(&str, &str)]) {}
    fn warn(&self, _msg: &str, _fields: &[(&str, &str)]) {}
    fn error(&self, _msg: &str, _fields: &[(&str, &str)]) {}
}

impl Metrics for Silent {
    fn increment(&self, _name: &str, _value: u64, _tags: &[(&str, &str)]) {}
    fn distribution(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}
}

/// Maps the lane DSN (database `hive_test`) onto this nextest worker's `hive_test_<slot>`.
fn worker_url(url: &str) -> String {
    let (head, tail) = url.rsplit_once('/').expect("DSN names a database");
    let (name, query) = tail.split_once('?').map_or((tail, ""), |(n, q)| (n, q));
    assert_eq!(
        name, "hive_test",
        "db tests use hive_test (scripts/test-db.sh)"
    );
    let slot = std::env::var("NEXTEST_TEST_GROUP_SLOT")
        .ok()
        .and_then(|slot| slot.parse::<u8>().ok())
        .unwrap_or(0);
    let sep = if query.is_empty() { "" } else { "?" };
    format!("{head}/hive_test_{slot}{sep}{query}")
}

async fn cell_router() -> axum::Router {
    let migrator_url = worker_url(
        &std::env::var("LOADS_MIGRATOR_DATABASE_URL")
            .expect("LOADS_MIGRATOR_DATABASE_URL is required"),
    );
    let cell_url =
        worker_url(&std::env::var("LOADS_DATABASE_URL").expect("LOADS_DATABASE_URL is required"));
    let mut options = ConnectOptions::new(migrator_url);
    options
        .max_connections(1)
        .set_schema_search_path(loads::SCHEMA);
    let migrator = Database::connect(options)
        .await
        .expect("migrator DSN unreachable");
    migrator
        .execute_unprepared("SELECT pg_advisory_lock(20260912)")
        .await
        .expect("migration lock");
    let migrated = loads::Migrator::up(&migrator, None).await;
    let truncated = migrator
        .execute_unprepared(
            "TRUNCATE TABLE loads.load, loads.stop, loads.idempotency_key RESTART IDENTITY CASCADE",
        )
        .await;
    migrator
        .execute_unprepared("SELECT pg_advisory_unlock(20260912)")
        .await
        .expect("migration unlock");
    migrated.expect("loads migrations");
    truncated.expect("truncate loads tables");
    migrator.close().await.expect("close migrator");
    let connection = Database::connect(cell_url)
        .await
        .expect("cell-role DSN unreachable");
    let clock = FakeClock::new(Instant::from_unix_timestamp(1_700_000_000).unwrap());
    let cell = loads::new(loads::LoadsPool::new(connection), clock, Silent, Silent);
    loads::router(&cell).split_for_parts().0
}

fn body() -> String {
    serde_json::json!({
        "shipperId": "shipper-1",
        "stops": [
            {
                "kind": "pickup",
                "date": "2026-09-20",
                "address": {
                    "line1": "1 Dock",
                    "city": "Dallas",
                    "region": "TX",
                    "postalCode": "75201",
                    "country": "US"
                }
            },
            {
                "kind": "delivery",
                "date": "2026-09-21",
                "name": "Consignee",
                "address": {
                    "line1": "9 Warehouse",
                    "city": "Austin",
                    "region": "TX",
                    "postalCode": "78701",
                    "country": "US"
                }
            }
        ]
    })
    .to_string()
}

fn with_identity(mut request: Request<Body>) -> Request<Body> {
    request
        .extensions_mut()
        .insert(loads::ActorId("actor-1".to_owned()));
    request
        .extensions_mut()
        .insert(loads::CorrelationId("corr-1".to_owned()));
    request
}

#[tokio::test]
async fn post_loads_without_idempotency_key_is_validation_failed() {
    let response = cell_router()
        .await
        .oneshot(with_identity(
            Request::builder()
                .method("POST")
                .uri("/loads")
                .header("content-type", "application/json")
                .body(Body::from(body()))
                .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap(),
        "application/problem+json"
    );
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["type"], "VALIDATION_FAILED");
    assert_eq!(json["status"], 400);
    assert_eq!(json["violations"][0]["path"], "Idempotency-Key");
    assert!(json.get("title").is_none());
    assert!(json.get("context").is_none());
}

#[tokio::test]
async fn post_loads_returns_the_load() {
    let key = Uuid::now_v7().to_string();
    let response = cell_router()
        .await
        .oneshot(with_identity(
            Request::builder()
                .method("POST")
                .uri("/loads")
                .header("Idempotency-Key", &key)
                .header("content-type", "application/json")
                .body(Body::from(body()))
                .unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = axum::body::to_bytes(response.into_body(), 64 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(json["shipperId"], "shipper-1");
    assert_eq!(json["createdAt"], "2023-11-14T22:13:20.000Z");
    assert_eq!(json["stops"].as_array().unwrap().len(), 2);
    assert!(json.get("actorId").is_none());
    assert!(Uuid::parse_str(json["id"].as_str().unwrap()).is_ok());
}
