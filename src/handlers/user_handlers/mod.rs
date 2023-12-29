

use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Json;
use crate::model::{UserToCreate, User};
use crate::utils::encrypt;
use crate::repositories::user_repository::{save_user, get_user};

pub async fn create_user_handler(data: Json<UserToCreate>) -> impl IntoResponse {

    let user_to_create = data.0;

    if user_to_create.password != user_to_create.pass_confirm {
        return Html(format!("passwords do not match"));
    }
    let encrypted_password = encrypt( user_to_create.password, &user_to_create.username);
    let user_to_save = User::new(user_to_create.username.clone(), user_to_create.email, encrypted_password);

    match save_user(user_to_save).await {
        Ok(user) => {
            Html(format!("User saved successfuly! {:?}", user))
        }
        Err(err) => {
            Html(format!("There was an error saving the new user: {:?}", err))
        }
    }
}

pub async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    match get_user(&id).await {
        Ok(user) => {
            if user.is_none() {
                return Html(format!("User not found"));
            }
            Html(format!("Success! {:?}", user.unwrap()))
        }
        Err(err) => {
            Html(format!("There was an error getting the user: {:?}", err))
        }
    }
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


