use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, Pool};

use crate::fmt::{print_console, ConsoleOutputTypes};

pub async fn create_pool_postgresql(database_url: &str) -> Pool<sqlx::Postgres> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            print_console(
                &format!("Error connecting to PostgreSQL database: {}", e),
                ConsoleOutputTypes::ERROR,
            );
            std::process::exit(1);
        }
    };
    pool
}

pub async fn create_pool_mysql(database_url: &str) -> Pool<sqlx::MySql> {
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            print_console(
                &format!("Error connecting to MySQL database: {}", e),
                ConsoleOutputTypes::ERROR,
            );
            std::process::exit(1);
        }
    };
    pool
}

pub async fn create_pool_sqlite(database_url: &str) -> Pool<sqlx::Sqlite> {
    let pool = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            print_console(
                &format!("Error connecting to SQLite database: {}", e),
                ConsoleOutputTypes::ERROR,
            );
            std::process::exit(1);
        }
    };
    pool
}