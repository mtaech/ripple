use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ApiLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiLog::Path).string().not_null())
                    .col(ColumnDef::new(ApiLog::ReqTime).string().not_null())
                    .col(ColumnDef::new(ApiLog::IpAddr).string().not_null())
                    .col(ColumnDef::new(ApiLog::UserAgent).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(ApiLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApiLog {
    Table,
    Id,
    Path,
    ReqTime,
    IpAddr,
    UserAgent,
}
