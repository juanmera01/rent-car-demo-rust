use axum::extract::{Path};
use axum::response::{Html, IntoResponse};

pub async fn create_vehicle_handler() -> impl IntoResponse {
    Html("sample post vehicle response")
}

pub async fn get_vehicle_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample get vehicle response {}", id))
}

pub async fn update_vehicle_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample update vehicle response {}", id))
}

pub async fn delete_vehicle_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample delete vehicle response {}", id))
}

pub async fn list_vehicles_handler() -> impl IntoResponse {
    Html("sample get vehicles response")
}