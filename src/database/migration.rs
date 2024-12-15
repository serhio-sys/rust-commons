use std::path::Path;
use log;

use chrono::NaiveDateTime;
use diesel::{
    migration::{ Migration, MigrationSource },
    pg::Pg,
    r2d2::{ ConnectionManager, Pool },
    PgConnection,
};
use diesel_migrations::{ FileBasedMigrations, MigrationHarness };

use crate::config::database_config::DatabaseConfig;

const DATE_FORMAT: &str = "%Y-%m-%d-%H%M%S";

pub fn migrate<T: DatabaseConfig>(
    db_conf: &T
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let manager = ConnectionManager::<PgConnection>::new(
        &format!(
            "postgres://{}:{}@{}/{}?sslmode=disable",
            db_conf.get_db_user(),
            db_conf.get_db_password(),
            db_conf.get_db_host(),
            db_conf.get_db_name()
        )
    );
    let pool = Pool::builder()
        .max_size(5)
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .unwrap_or_else(|_| panic!("Error: Unable to establish database connection."));

    let binding = db_conf.get_migrations_location().to_string();
    let migration_path = Path::new(binding.as_str());
    let migrator = diesel_migrations::FileBasedMigrations
        ::find_migrations_directory_in_path(migration_path)
        .map_err(|err| format!("Error in creating FileBasedMigrations: {}", err))
        .unwrap();
    //conn.revert_all_migrations(migrator.clone()).expect("Could not revert migrations");
    if db_conf.get_migrations_version() != "latest".into() {
        migrate_to_version(&migrator, &pool, db_conf.get_migrations_version().to_string().as_str());
    } else {
        let is_pending_migrations = !MigrationHarness::has_pending_migration(
            &mut pool.get().unwrap(),
            migrator.clone()
        ).unwrap();
        if is_pending_migrations {
            log::info!("Nothing to migrate");
        } else {
            MigrationHarness::run_pending_migrations(&mut pool.get().unwrap(), migrator)?;
            log::info!("Migrated successfully to latest migration");
        }
    }
    log::info!("Migrations was passed");
    return Ok(());
}

fn migrate_to_version(
    migrator: &FileBasedMigrations,
    connection: &Pool<ConnectionManager<PgConnection>>,
    migration_version: &str
) {
    let parsed_date = NaiveDateTime::parse_from_str(migration_version, DATE_FORMAT);
    if parsed_date.is_err() {
        panic!("Migration version is not parseble. Check migration version");
    }
    let migrations: Vec<Box<dyn Migration<Pg>>> = FileBasedMigrations::migrations(
        migrator
    ).unwrap();
    for migration in migrations {
        let name = migration.name().to_string();
        if name.contains("diesel_initial_setup") {
            continue;
        }
        let splitted_name: Vec<&str> = name.split("_").collect();
        match NaiveDateTime::parse_from_str(splitted_name.first().unwrap(), DATE_FORMAT) {
            Ok(parsed_datetime) => {
                if parsed_datetime <= parsed_date.unwrap() {
                    if
                        let Err(e) = MigrationHarness::run_migration(
                            &mut connection.get().unwrap(),
                            &migration
                        )
                    {
                        log::error!("Run migration error: {}", e.to_string());
                    } else {
                        log::info!("Executed migration: {}", name);
                    }
                } else {
                    if
                        let Err(e) = MigrationHarness::revert_migration(
                            &mut connection.get().unwrap(),
                            &migration
                        )
                    {
                        log::error!("Revert migration error: {}", e.to_string());
                    } else {
                        log::info!("Reverted migration: {}", name);
                    }
                }
            }
            Err(e) => {
                log::error!("Error parsing date: {}", e);
            }
        }
    }
}
