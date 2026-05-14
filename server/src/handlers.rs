use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;
use validator::Validate;

use crate::{
    errors::AppError,
    models::{CreatePointDto, Point},
};

pub async fn create_point(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreatePointDto>,
) -> Result<Json<Point>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let result = sqlx::query(
        r#"
        INSERT INTO points (lng, lat, camp)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(payload.lng)
    .bind(payload.lat)
    .bind(payload.camp)
    .execute(&pool)
    .await?;

    let id = result.last_insert_rowid();

    Ok(Json(Point {
        id,
        lng: payload.lng,
        lat: payload.lat,
        camp: payload.camp,
    }))
}

pub async fn get_points(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Point>>, AppError> {
    let points = sqlx::query_as::<_, Point>(
        r#"
        SELECT id, lng, lat, camp
        FROM points
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(points))
}