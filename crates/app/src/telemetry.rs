use kernel::{Logger, Metrics};
use opentelemetry::trace::TracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub struct TracingLogger;

impl Logger for TracingLogger {
    fn debug(&self, msg: &str, fields: &[(&str, &str)]) {
        tracing::debug!(msg, fields = ?fields);
    }

    fn info(&self, msg: &str, fields: &[(&str, &str)]) {
        tracing::info!(msg, fields = ?fields);
    }

    fn warn(&self, msg: &str, fields: &[(&str, &str)]) {
        tracing::warn!(msg, fields = ?fields);
    }

    fn error(&self, msg: &str, fields: &[(&str, &str)]) {
        tracing::error!(msg, fields = ?fields);
    }
}

pub struct AppMetrics;

impl Metrics for AppMetrics {
    fn increment(&self, name: &str, value: u64, tags: &[(&str, &str)]) {
        let meter = opentelemetry::global::meter("hive");
        let counter = meter.u64_counter(name.to_owned()).build();
        counter.add(value, &key_values(tags));
    }

    fn distribution(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        let meter = opentelemetry::global::meter("hive");
        let histogram = meter.f64_histogram(name.to_owned()).build();
        histogram.record(value, &key_values(tags));
    }
}

fn key_values(tags: &[(&str, &str)]) -> Vec<opentelemetry::KeyValue> {
    tags.iter()
        .map(|(key, value)| opentelemetry::KeyValue::new(key.to_string(), value.to_string()))
        .collect()
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let filter = std::env::var("LOG_LEVEL")
        .ok()
        .and_then(|level| EnvFilter::try_new(level).ok())
        .unwrap_or_else(|| EnvFilter::new("info"));
    let fmt_layer = fmt::layer().json();

    if std::env::var_os("OTEL_EXPORTER_OTLP_ENDPOINT").is_some() {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()?;
        let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .build();
        let tracer = provider.tracer("app");
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(otel_layer)
            .init();
        opentelemetry::global::set_tracer_provider(provider);
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{AppMetrics, TracingLogger};
    use kernel::{Logger, Metrics};

    #[test]
    fn logger_and_metrics_impls_bind() {
        fn assert_logger<L: Logger>() {}
        fn assert_metrics<M: Metrics>() {}
        assert_logger::<TracingLogger>();
        assert_metrics::<AppMetrics>();
    }
}
