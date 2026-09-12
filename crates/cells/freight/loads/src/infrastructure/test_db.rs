use crate::infrastructure::LoadsPool;
use crate::migrations::{Migrator, SCHEMA};
use sea_orm::{ConnectOptions, ConnectionTrait, Database};
use sea_orm_migration::MigratorTrait;

const MIGRATE_LOCK: &str = "SELECT pg_advisory_lock(20260912)";
const MIGRATE_UNLOCK: &str = "SELECT pg_advisory_unlock(20260912)";
const TRUNCATE: &str =
    "TRUNCATE TABLE loads.load, loads.stop, loads.idempotency_key RESTART IDENTITY CASCADE";

pub(crate) async fn migrated_pool() -> LoadsPool {
    let migrator_url = worker_url(
        &std::env::var("LOADS_MIGRATOR_DATABASE_URL")
            .expect("LOADS_MIGRATOR_DATABASE_URL is required"),
    );
    let cell_url =
        worker_url(&std::env::var("LOADS_DATABASE_URL").expect("LOADS_DATABASE_URL is required"));
    let mut options = ConnectOptions::new(migrator_url);
    options.max_connections(1).set_schema_search_path(SCHEMA);
    let migrator = Database::connect(options)
        .await
        .expect("migrator DSN unreachable");
    migrator
        .execute_unprepared(MIGRATE_LOCK)
        .await
        .expect("migration lock");
    let migrated = Migrator::up(&migrator, None).await;
    let truncated = migrator.execute_unprepared(TRUNCATE).await;
    migrator
        .execute_unprepared(MIGRATE_UNLOCK)
        .await
        .expect("migration unlock");
    migrated.expect("loads migrations");
    truncated.expect("truncate loads tables");
    migrator.close().await.expect("close migrator");
    LoadsPool::new(
        Database::connect(cell_url)
            .await
            .expect("cell-role DSN unreachable"),
    )
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
