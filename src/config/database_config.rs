use std::sync::Arc;

pub trait DatabaseConfig {
    fn get_db_host(&self) -> Arc<str>;

    fn get_db_name(&self) -> Arc<str>;

    fn get_db_password(&self) -> Arc<str>;

    fn get_db_user(&self) -> Arc<str>;

    fn get_migrations_location(&self) -> Arc<str>;

    fn get_migrations_version(&self) -> Arc<str>;
}
