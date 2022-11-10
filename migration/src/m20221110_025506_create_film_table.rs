use sea_orm::{ColumnDef, DbErr, sea_query};
use sea_orm::sea_query::SimpleExpr::Column;
use entity::category::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Label).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).to_owned())
            .await
    }
}