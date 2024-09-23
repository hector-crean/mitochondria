use axum::Json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ScreenshotRequest {}

#[derive(Serialize)]
pub struct ScreenshotResponse {}

pub async fn screenshot(Json(payload): Json<ScreenshotRequest>) -> Json<ScreenshotResponse> {
    Json(ScreenshotResponse {})
}
