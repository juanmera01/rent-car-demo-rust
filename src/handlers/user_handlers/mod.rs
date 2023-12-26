

use axum::extract::{Path};
use axum::response::{Html, IntoResponse};

pub async fn create_user_handler() -> impl IntoResponse {
    Html("sample post user response")
}

pub async fn get_user_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample get user response {}", id))
}

pub async fn update_user_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample update user response {}", id))
}

pub async fn delete_user_handler(Path(id): Path<i32>) -> impl IntoResponse {
    Html(format!("sample delete user response {}", id))
}

pub async fn list_users_handler() -> impl IntoResponse {
    Html("sample get users response")
}

// -------------------------



// --------------------------

