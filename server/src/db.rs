use sqlx::{
    sqlite::SqlitePoolOptions,
    SqlitePool,
};

pub async fn create_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://points.db")
        .await
        .expect("DB connect failed");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS points (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            lng REAL NOT NULL,
            lat REAL NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Create table failed");

    pool
}