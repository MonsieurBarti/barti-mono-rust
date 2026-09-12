use super::SCHEMA;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub(crate) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Alias::new(SCHEMA), Load::Table))
                    .col(ColumnDef::new(Load::Id).uuid().primary_key())
                    .col(ColumnDef::new(Load::ShipperId).text().not_null())
                    .col(ColumnDef::new(Load::ActorId).text().not_null())
                    .col(
                        ColumnDef::new(Load::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Alias::new(SCHEMA), Stop::Table))
                    .col(ColumnDef::new(Stop::Id).uuid().primary_key())
                    .col(ColumnDef::new(Stop::LoadId).uuid().not_null())
                    .col(ColumnDef::new(Stop::Kind).text().not_null())
                    .col(ColumnDef::new(Stop::Date).date().not_null())
                    .col(ColumnDef::new(Stop::Name).text())
                    .col(ColumnDef::new(Stop::Line1).text().not_null())
                    .col(ColumnDef::new(Stop::Line2).text())
                    .col(ColumnDef::new(Stop::City).text().not_null())
                    .col(ColumnDef::new(Stop::Region).text().not_null())
                    .col(ColumnDef::new(Stop::PostalCode).text().not_null())
                    .col(ColumnDef::new(Stop::Country).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from((Alias::new(SCHEMA), Stop::Table), Stop::LoadId)
                            .to((Alias::new(SCHEMA), Load::Table), Load::Id),
                    )
                    .index(
                        Index::create()
                            .name("stop_load_id_kind_key")
                            .unique()
                            .col(Stop::LoadId)
                            .col(Stop::Kind),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table((Alias::new(SCHEMA), IdempotencyKey::Table))
                    .col(ColumnDef::new(IdempotencyKey::ActorId).text().not_null())
                    .col(ColumnDef::new(IdempotencyKey::Key).text().not_null())
                    .col(
                        ColumnDef::new(IdempotencyKey::Fingerprint)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IdempotencyKey::Outcome).text().not_null())
                    .primary_key(
                        Index::create()
                            .col(IdempotencyKey::ActorId)
                            .col(IdempotencyKey::Key),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new(SCHEMA), IdempotencyKey::Table))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new(SCHEMA), Stop::Table))
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table((Alias::new(SCHEMA), Load::Table))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Load {
    Table,
    Id,
    ShipperId,
    ActorId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub(crate) enum Stop {
    Table,
    Id,
    LoadId,
    Kind,
    Date,
    Name,
    Line1,
    Line2,
    City,
    Region,
    PostalCode,
    Country,
}

#[derive(DeriveIden)]
pub(crate) enum IdempotencyKey {
    Table,
    ActorId,
    Key,
    Fingerprint,
    Outcome,
}
