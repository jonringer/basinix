pub mod models;

use log::info;
use std::env;

const LOG_TARGET: &str = "basinix::shared::db";

use sqlx::Executor;

pub async fn init_database(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    let branch_table =
        sqlx::query!("SELECT name FROM sqlite_master WHERE type='table' AND name='branch';")
            .fetch_one(pool)
            .await;
    match branch_table {
        Ok(_) => {
            info!(target: LOG_TARGET, "Database has already been initialized");
            Ok(())
        }
        Err(_) => {
            info!(target: LOG_TARGET, "Initializing database");
            let sql_initialization = include_str!("db/sql/up.sql");
            pool.execute(sql_initialization)
                .await
                .expect("Unable to initial database.");
            info!(target: LOG_TARGET, "Database initialized");
            Ok(())
        }
    }
}

pub async fn create_connection_pool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    if database_url != "sqlite::memory:" {
        let db_path = std::path::Path::new(&database_url);
        if !db_path.exists() {
            info!(
                target: LOG_TARGET,
                "{} not found, creating new database",
                db_path.to_str().unwrap()
            );
            if !db_path.parent().unwrap().is_dir() {
                std::fs::create_dir_all(db_path.parent().unwrap())
                    .expect("Unable to write to create database directory");
            }
            std::fs::File::create(db_path).expect(&format!(
                "Unable to create database at {}",
                db_path.to_str().unwrap()
            ));
        }
    }

    info!(target: LOG_TARGET, "Creating database connection pool");
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5u32)
        .connect(&database_url)
        .await
}
