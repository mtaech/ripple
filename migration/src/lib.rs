pub use sea_orm_migration::prelude::*;

mod m20240229_115556_create_table_user;
mod m20240229_123011_create_table_api_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240229_115556_create_table_user::Migration),
            Box::new(m20240229_123011_create_table_api_post::Migration),
        ]
    }
}
