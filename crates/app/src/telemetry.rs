use kernel::{Logger, Metrics};
use opentelemetry::metrics::Meter;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

pub(crate) struct TracingLogger;

macro_rules! emit {
    ($level:ident, $msg:expr, $fields:expr) => {{
        let get = |key: &str| {
            $fields
                .iter()
                .copied()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v)
        };
        tracing::$level!(
            msg = $msg,
            correlationId = get("correlationId"),
            actorId = get("actorId"),
            source = get("source"),
            duration_ms = get("duration_ms"),
            status = get("status"),
            method = get("method"),
            path = get("path"),
            r#type = get("type"),
        );
    }};
}

impl Logger for TracingLogger {
    fn debug(&self, msg: &str, fields: &[(&str, &str)]) {
        emit!(debug, msg, fields);
    }

    fn info(&self, msg: &str, fields: &[(&str, &str)]) {
        emit!(info, msg, fields);
    }

    fn warn(&self, msg: &str, fields: &[(&str, &str)]) {
        emit!(warn, msg, fields);
    }

    fn error(&self, msg: &str, fields: &[(&str, &str)]) {
        emit!(error, msg, fields);
    }
}

pub(crate) struct AppMetrics {
    meter: Meter,
}

impl AppMetrics {
    pub(crate) fn new() -> Self {
        Self {
            meter: opentelemetry::global::meter("hive"),
        }
    }
}

impl Metrics for AppMetrics {
    fn increment(&self, name: &str, value: u64, tags: &[(&str, &str)]) {
        let counter = self.meter.u64_counter(name.to_owned()).build();
        counter.add(value, &key_values(tags));
    }

    fn distribution(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        let histogram = self.meter.f64_histogram(name.to_owned()).build();
        histogram.record(value, &key_values(tags));
    }
}

fn key_values(tags: &[(&str, &str)]) -> Vec<opentelemetry::KeyValue> {
    tags.iter()
        .map(|(key, value)| opentelemetry::KeyValue::new(key.to_string(), value.to_string()))
        .collect()
}

fn log_filter(raw: Option<String>) -> EnvFilter {
    match raw {
        Some(level) if !level.is_empty() => {
            EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("info"))
        }
        _ => EnvFilter::new("info"),
    }
}

pub(crate) struct Telemetry {
    tracer: Option<SdkTracerProvider>,
    meter: Option<SdkMeterProvider>,
}

impl Drop for Telemetry {
    fn drop(&mut self) {
        if let Some(tracer) = self.tracer.take() {
            let _ = tracer.shutdown();
        }
        if let Some(meter) = self.meter.take() {
            let _ = meter.shutdown();
        }
    }
}

pub(crate) fn init() -> Result<Telemetry, Box<dyn std::error::Error>> {
    let filter = log_filter(std::env::var("LOG_LEVEL").ok());
    let fmt_layer = fmt::layer().json();

    if std::env::var_os("OTEL_EXPORTER_OTLP_ENDPOINT").is_some() {
        let span_exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .build()?;
        let tracer = SdkTracerProvider::builder()
            .with_batch_exporter(span_exporter)
            .build();
        let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_http()
            .build()?;
        let meter = SdkMeterProvider::builder()
            .with_periodic_exporter(metric_exporter)
            .build();
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer.tracer("app"));
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .with(otel_layer)
            .init();
        opentelemetry::global::set_tracer_provider(tracer.clone());
        opentelemetry::global::set_meter_provider(meter.clone());
        Ok(Telemetry {
            tracer: Some(tracer),
            meter: Some(meter),
        })
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
        Ok(Telemetry {
            tracer: None,
            meter: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AppMetrics, TracingLogger, log_filter};
    use kernel::{Logger, Metrics};
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::EnvFilter;

    #[test]
    fn logger_and_metrics_impls_bind() {
        fn assert_logger<L: Logger>() {}
        fn assert_metrics<M: Metrics>() {}
        assert_logger::<TracingLogger>();
        assert_metrics::<AppMetrics>();
    }

    #[test]
    fn log_filter_empty_or_unset_is_info() {
        let info = EnvFilter::new("info").to_string();
        assert_eq!(log_filter(None).to_string(), info);
        assert_eq!(log_filter(Some(String::new())).to_string(), info);
        assert!(!log_filter(None).to_string().is_empty());
        assert!(!log_filter(Some(String::new())).to_string().is_empty());
    }

    struct Buf(Arc<Mutex<Vec<u8>>>);

    impl Write for Buf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().expect("buf").extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn tracing_logger_emits_named_json_fields() {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let writer = {
            let bytes = bytes.clone();
            move || Buf(bytes.clone())
        };
        let subscriber = tracing_subscriber::fmt()
            .json()
            .with_writer(writer)
            .finish();
        tracing::subscriber::with_default(subscriber, || {
            TracingLogger.info("hello", &[("source", "api"), ("correlationId", "x")]);
        });
        let line = String::from_utf8(bytes.lock().expect("buf").clone()).expect("utf8");
        let v: serde_json::Value = serde_json::from_str(line.trim()).expect("json");
        let fields = &v["fields"];
        assert_eq!(fields["msg"], "hello");
        assert_eq!(fields["source"], "api");
        assert_eq!(fields["correlationId"], "x");
        assert!(
            fields
                .as_object()
                .expect("object")
                .values()
                .all(|value| !value.as_str().is_some_and(|s| s.starts_with("[(")))
        );
    }
}
