use axum::extract::{Path};
use axum::response::{Html, IntoResponse};

pub async fn index_handler() -> impl IntoResponse {
    Html("sample index response")
}

pub async fn login_handler() -> impl IntoResponse {
    Html("sample login response")
}

pub async fn logout_handler() -> impl IntoResponse {
    Html("sample logout response")
}

pub async fn signup_handler() -> impl IntoResponse {
    Html("sample signup response")
}