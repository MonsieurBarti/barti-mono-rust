use sqlx::PgPool;

pub(crate) struct LoadsPool(PgPool);

impl LoadsPool {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub(crate) fn inner(&self) -> &PgPool {
        &self.0
    }
}
