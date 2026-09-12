use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, RuntimeErr, Statement,
};

const PROBE: &str = "SELECT id FROM grant_proof.probe";

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

async fn connect(var: &str) -> DatabaseConnection {
    let url = std::env::var(var).unwrap_or_else(|_| panic!("{var} is required"));
    Database::connect(worker_url(&url))
        .await
        .unwrap_or_else(|err| panic!("{var} unreachable: {err}"))
}

fn sqlstate(err: &DbErr) -> Option<String> {
    match err {
        DbErr::Exec(RuntimeErr::SqlxError(e)) | DbErr::Query(RuntimeErr::SqlxError(e)) => {
            match &**e {
                sea_orm::sqlx::Error::Database(db) => db.code().map(|code| code.into_owned()),
                _ => None,
            }
        }
        _ => None,
    }
}

#[tokio::test]
async fn loads_role_cannot_read_a_planted_foreign_schema() {
    let owner = connect("GRANT_PROOF_DATABASE_URL").await;
    owner
        .query_all_raw(Statement::from_string(DbBackend::Postgres, PROBE))
        .await
        .expect("grant_proof owner reads grant_proof.probe");

    let loads = connect("LOADS_DATABASE_URL").await;
    let err = loads
        .query_all_raw(Statement::from_string(DbBackend::Postgres, PROBE))
        .await
        .expect_err("loads role must not read grant_proof.probe");
    assert_eq!(sqlstate(&err).as_deref(), Some("42501"), "{err}");
}
