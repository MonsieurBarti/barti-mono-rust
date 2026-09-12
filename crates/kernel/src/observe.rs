pub trait Logger: Send + Sync {
    fn debug(&self, msg: &str, fields: &[(&str, &str)]);
    fn info(&self, msg: &str, fields: &[(&str, &str)]);
    fn warn(&self, msg: &str, fields: &[(&str, &str)]);
    fn error(&self, msg: &str, fields: &[(&str, &str)]);
}

pub trait Metrics: Send + Sync {
    fn increment(&self, name: &str, value: u64, tags: &[(&str, &str)]);
    fn distribution(&self, name: &str, value: f64, tags: &[(&str, &str)]);
}

#[cfg(test)]
mod tests {
    use super::{Logger, Metrics};

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

    #[test]
    fn logger_and_metrics_are_ports() {
        fn assert_logger<L: Logger>() {}
        fn assert_metrics<M: Metrics>() {}
        assert_logger::<Silent>();
        assert_metrics::<Silent>();
    }
}
