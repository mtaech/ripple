use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Nickname)
                            .string()
                            .not_null()
                            .comment("昵称"),
                    )
                    .col(
                        ColumnDef::new(User::LoginName)
                            .string()
                            .not_null()
                            .comment("登录名"),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .not_null()
                            .comment("邮箱"),
                    )
                    .col(
                        ColumnDef::new(User::Password)
                            .string()
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(User::Status)
                            .integer()
                            .comment("状态：0：未激活，1：激活 2：禁用"),
                    )
                    .col(
                        ColumnDef::new(User::UserType)
                            .integer()
                            .comment("用户类型：1：管理员 2：普通用户"),
                    )
                    .col(ColumnDef::new(User::CreateTime).date_time())
                    .col(ColumnDef::new(User::UpdateTime).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Nickname,
    LoginName,
    Email,
    Password,
    Status,
    UserType,
    CreateTime,
    UpdateTime,
}
