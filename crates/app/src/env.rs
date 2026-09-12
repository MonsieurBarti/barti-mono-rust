use std::fmt;

#[derive(Debug)]
pub struct BootError {
    var: &'static str,
    kind: BootErrorKind,
}

#[derive(Debug)]
enum BootErrorKind {
    Missing,
    Invalid,
}

impl BootError {
    fn missing(var: &'static str) -> Self {
        Self {
            var,
            kind: BootErrorKind::Missing,
        }
    }

    fn invalid(var: &'static str) -> Self {
        Self {
            var,
            kind: BootErrorKind::Invalid,
        }
    }
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            BootErrorKind::Missing => write!(f, "{} is required", self.var),
            BootErrorKind::Invalid => write!(f, "{} is invalid", self.var),
        }
    }
}

impl std::error::Error for BootError {}

#[derive(Debug)]
pub struct MigrateEnv {
    pub loads_migrator_database_url: String,
}

impl MigrateEnv {
    pub fn from_get(get: impl Fn(&str) -> Option<String>) -> Result<Self, BootError> {
        let loads_migrator_database_url = required(&get, "LOADS_MIGRATOR_DATABASE_URL")?;
        Ok(Self {
            loads_migrator_database_url,
        })
    }
}

#[derive(Debug)]
pub struct ServeEnv {
    pub loads_database_url: String,
    pub loads_pool_max: u32,
}

impl ServeEnv {
    pub fn from_get(get: impl Fn(&str) -> Option<String>) -> Result<Self, BootError> {
        let loads_database_url = required(&get, "LOADS_DATABASE_URL")?;
        let loads_pool_max = required(&get, "LOADS_POOL_MAX")?
            .parse()
            .map_err(|_| BootError::invalid("LOADS_POOL_MAX"))?;
        if loads_pool_max == 0 {
            return Err(BootError::invalid("LOADS_POOL_MAX"));
        }
        Ok(Self {
            loads_database_url,
            loads_pool_max,
        })
    }
}

fn required(get: impl Fn(&str) -> Option<String>, var: &'static str) -> Result<String, BootError> {
    get(var)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| BootError::missing(var))
}

#[cfg(test)]
mod tests {
    use super::{BootErrorKind, MigrateEnv, ServeEnv};

    fn get<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |key| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| (*v).to_string())
        }
    }

    #[test]
    fn migrate_requires_migrator_dsn() {
        let err = MigrateEnv::from_get(get(&[])).unwrap_err();
        assert_eq!(err.var, "LOADS_MIGRATOR_DATABASE_URL");
        assert!(matches!(err.kind, BootErrorKind::Missing));
    }

    #[test]
    fn migrate_reads_only_the_migrator_dsn() {
        let env = MigrateEnv::from_get(get(&[(
            "LOADS_MIGRATOR_DATABASE_URL",
            "postgres://loads_migrator@localhost/hive",
        )]))
        .unwrap();
        assert_eq!(
            env.loads_migrator_database_url,
            "postgres://loads_migrator@localhost/hive"
        );
    }

    #[test]
    fn migrate_ignores_cell_role_dsn_and_pool_max() {
        let env = MigrateEnv::from_get(get(&[
            (
                "LOADS_MIGRATOR_DATABASE_URL",
                "postgres://loads_migrator@localhost/hive",
            ),
            ("LOADS_DATABASE_URL", "postgres://loads@localhost/hive"),
            ("LOADS_POOL_MAX", "not-a-number"),
        ]))
        .unwrap();
        assert_eq!(
            env.loads_migrator_database_url,
            "postgres://loads_migrator@localhost/hive"
        );
    }

    #[test]
    fn migrate_rejects_empty_migrator_dsn() {
        let err = MigrateEnv::from_get(get(&[("LOADS_MIGRATOR_DATABASE_URL", "")])).unwrap_err();
        assert_eq!(err.var, "LOADS_MIGRATOR_DATABASE_URL");
    }

    #[test]
    fn serve_requires_cell_role_dsn() {
        let err = ServeEnv::from_get(get(&[("LOADS_POOL_MAX", "10")])).unwrap_err();
        assert_eq!(err.var, "LOADS_DATABASE_URL");
        assert!(matches!(err.kind, BootErrorKind::Missing));
    }

    #[test]
    fn serve_requires_pool_max() {
        let err = ServeEnv::from_get(get(&[(
            "LOADS_DATABASE_URL",
            "postgres://loads@localhost/hive",
        )]))
        .unwrap_err();
        assert_eq!(err.var, "LOADS_POOL_MAX");
        assert!(matches!(err.kind, BootErrorKind::Missing));
    }

    #[test]
    fn serve_rejects_unparseable_pool_max() {
        let err = ServeEnv::from_get(get(&[
            ("LOADS_DATABASE_URL", "postgres://loads@localhost/hive"),
            ("LOADS_POOL_MAX", "nope"),
        ]))
        .unwrap_err();
        assert_eq!(err.var, "LOADS_POOL_MAX");
        assert!(matches!(err.kind, BootErrorKind::Invalid));
    }

    #[test]
    fn serve_rejects_zero_pool_max() {
        let err = ServeEnv::from_get(get(&[
            ("LOADS_DATABASE_URL", "postgres://loads@localhost/hive"),
            ("LOADS_POOL_MAX", "0"),
        ]))
        .unwrap_err();
        assert_eq!(err.var, "LOADS_POOL_MAX");
        assert!(matches!(err.kind, BootErrorKind::Invalid));
    }

    #[test]
    fn serve_reads_cell_role_dsn_and_pool_max() {
        let env = ServeEnv::from_get(get(&[
            ("LOADS_DATABASE_URL", "postgres://loads@localhost/hive"),
            ("LOADS_POOL_MAX", "10"),
        ]))
        .unwrap();
        assert_eq!(env.loads_database_url, "postgres://loads@localhost/hive");
        assert_eq!(env.loads_pool_max, 10);
    }

    #[test]
    fn serve_does_not_read_migrator_dsn() {
        let env = ServeEnv::from_get(get(&[
            ("LOADS_DATABASE_URL", "postgres://loads@localhost/hive"),
            ("LOADS_POOL_MAX", "10"),
            (
                "LOADS_MIGRATOR_DATABASE_URL",
                "postgres://loads_migrator@localhost/hive",
            ),
        ]))
        .unwrap();
        assert_eq!(env.loads_database_url, "postgres://loads@localhost/hive");
        assert_eq!(env.loads_pool_max, 10);
    }
}
