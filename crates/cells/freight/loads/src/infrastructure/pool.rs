use sqlx::PgPool;

pub struct LoadsPool(PgPool);

impl LoadsPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub(crate) fn inner(&self) -> &PgPool {
        &self.0
    }
}
