use crate::env::MigrateEnv;
use sqlx::postgres::PgPoolOptions;

pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let env = MigrateEnv::from_get(|key| std::env::var(key).ok())?;
    run_with(&env).await?;
    Ok(())
}

pub(crate) async fn run_with(env: &MigrateEnv) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&env.loads_migrator_database_url)
        .await?;
    sqlx::migrate!("../cells/freight/loads/migrations")
        .run(&pool)
        .await?;
    Ok(())
}
