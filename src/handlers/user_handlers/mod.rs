

use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Json;
use crate::model::{UserToCreate, User};
use crate::utils::{encrypt, decrypt};
use crate::repositories::user_repository::save_user;

pub async fn create_user_handler(data: Json<UserToCreate>) -> impl IntoResponse {

    let user_to_create = data.0;

    if user_to_create.password != user_to_create.pass_confirm {
        return Html("passwords do not match");
    }
    let encrypted_password = encrypt( user_to_create.password, user_to_create.username);
    let user_to_save = User::new(user_to_create.username, user_to_create.email, encrypted_password);

    match save_user(user_to_save).await {
        Ok(contents) => {
            Html("The user was created successfuly!")
        }
        Err(err) => {
            Html("There was an error saving the new user")
        }
    }
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


