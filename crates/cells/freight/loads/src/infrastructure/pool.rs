use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct LoadsPool(DatabaseConnection);

impl LoadsPool {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self(connection)
    }

    pub(crate) fn inner(&self) -> &DatabaseConnection {
        &self.0
    }
}
