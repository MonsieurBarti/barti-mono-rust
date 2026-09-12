use crate::env::ServeEnv;
use crate::http;
use crate::telemetry::{self, AppMetrics, TracingLogger};
use kernel::{Clock, Logger, Metrics, SystemClock};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub const BIND: &str = "127.0.0.1:8080";

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    telemetry::init()?;
    let env = ServeEnv::from_get(|key| std::env::var(key).ok())?;

    // Ticket 07 wraps this pool in the loads cell-private newtype, and ticket 08
    // passes the clock, logger, and metrics into `loads::new`.
    let loads_pool = loads_pool(&env.loads_database_url, env.loads_pool_max).await?;
    let clock = SystemClock;
    let logger = TracingLogger;
    let metrics = AppMetrics;

    logger.info(
        "app.serve.started",
        &[
            ("bind", BIND),
            ("source", "api"),
            ("startedAt", &format!("{:?}", clock.now())),
            (
                "loadsPoolSize",
                &loads_pool.options().get_max_connections().to_string(),
            ),
        ],
    );
    metrics.increment("hive.app.serve_started", 1, &[("cell", "loads")]);

    let listener = TcpListener::bind(BIND).await?;
    axum::serve(listener, http::router()).await?;
    Ok(())
}

/// The named pool for the `loads` cell. Never shared with another cell.
async fn loads_pool(database_url: &str, pool_max: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(pool_max)
        .connect(database_url)
        .await
}
