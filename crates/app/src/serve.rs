use crate::env::ServeEnv;
use crate::http;
use crate::telemetry::{self, AppMetrics, TracingLogger};
use kernel::{Clock, Logger, Metrics, SystemClock};
use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

pub(crate) const BIND: &str = "127.0.0.1:8080";

pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let telemetry = telemetry::init()?;
    let env = ServeEnv::from_get(|key| std::env::var(key).ok())?;

    let mut options = ConnectOptions::new(&env.loads_database_url);
    options
        .max_connections(env.loads_pool_max)
        .sqlx_logging(false);
    let loads_connection = Database::connect(options).await?;
    let clock = SystemClock;
    let logger = TracingLogger;
    let metrics = AppMetrics::new();
    logger.info(
        "app.serve.started",
        &[
            ("bind", BIND),
            ("source", "api"),
            ("startedAt", &format!("{:?}", clock.now())),
            ("loadsPoolSize", &env.loads_pool_max.to_string()),
        ],
    );
    metrics.increment("hive.app.serve_started", 1, &[("cell", "loads")]);
    let loads = loads::new(
        loads::LoadsPool::new(loads_connection),
        clock,
        logger,
        metrics,
    );

    let listener = TcpListener::bind(BIND).await?;
    axum::serve(
        listener,
        http::router(loads::router(&loads).split_for_parts().0),
    )
    .with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await?;
    drop(telemetry);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::BIND;

    #[test]
    fn bind_is_loopback_8080() {
        assert_eq!(BIND, "127.0.0.1:8080");
    }
}
