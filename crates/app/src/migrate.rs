use crate::env::MigrateEnv;
use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm_migration::MigratorTrait;

pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let env = MigrateEnv::from_get(|key| std::env::var(key).ok())?;
    run_with(&env).await?;
    Ok(())
}

pub(crate) async fn run_with(env: &MigrateEnv) -> Result<(), DbErr> {
    let mut options = ConnectOptions::new(&env.loads_migrator_database_url);
    options
        .max_connections(1)
        .sqlx_logging(false)
        .set_schema_search_path(loads::SCHEMA);
    let connection = Database::connect(options).await?;
    loads::Migrator::up(&connection, None).await?;
    connection.close().await
}
