/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! SQLite connection pool. Limit 5 connections to avoid locking issues.

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todo.db".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool().await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_pool_connections() {
        let pool = create_pool().await.unwrap();
        let conn = pool.acquire().await;
        assert!(conn.is_ok());
    }
}
