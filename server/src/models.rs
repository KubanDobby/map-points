use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    sqlx::Type,
)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum Camp {
    Friend,
    Enemy,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Point {
    pub id: i64,
    pub lng: f64,
    pub lat: f64,
    pub camp: Camp,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePointDto {
    #[validate(range(min = -180.0, max = 180.0))]
    pub lng: f64,

    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: f64,

    pub camp: Camp,
}