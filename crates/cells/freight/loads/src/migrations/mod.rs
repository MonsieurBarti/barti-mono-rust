use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20260912_120000_load_write_model;

/// This cell's Postgres schema. `app` passes it explicitly on the migrator connection.
pub const SCHEMA: &str = "loads";

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260912_120000_load_write_model::Migration)]
    }
}
