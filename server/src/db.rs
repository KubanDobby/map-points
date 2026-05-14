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

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    pool
}