use crate::infrastructure::LoadsPool;
use crate::migrations::{Migrator, SCHEMA};
use sea_orm::{ConnectOptions, ConnectionTrait, Database};
use sea_orm_migration::MigratorTrait;

const MIGRATE_LOCK: &str = "SELECT pg_advisory_lock(20260912)";
const MIGRATE_UNLOCK: &str = "SELECT pg_advisory_unlock(20260912)";

pub(crate) async fn migrated_pool() -> LoadsPool {
    let migrator_url = std::env::var("LOADS_MIGRATOR_DATABASE_URL")
        .expect("LOADS_MIGRATOR_DATABASE_URL is required");
    let cell_url = std::env::var("LOADS_DATABASE_URL").expect("LOADS_DATABASE_URL is required");
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
    migrator
        .execute_unprepared(MIGRATE_UNLOCK)
        .await
        .expect("migration unlock");
    migrated.expect("loads migrations");
    migrator.close().await.expect("close migrator");
    LoadsPool::new(
        Database::connect(cell_url)
            .await
            .expect("cell-role DSN unreachable"),
    )
}
