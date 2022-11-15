pub use sea_orm_migration::prelude::*;

mod create_category_table;
mod create_film_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_category_table::Migration),
            Box::new(create_film_table::Migration),
        ]
    }
}
