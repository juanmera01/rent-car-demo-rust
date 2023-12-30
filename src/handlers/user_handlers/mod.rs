

use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Json;
use crate::model::user::{UserToCreate, User};
use crate::utils::encrypt;
use crate::repositories::user_repository::{save_user, get_user, delete_user, list_users, update_user};

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

pub async fn update_user_handler(data: Json<User>) -> impl IntoResponse {
    let user = data.0;
    match get_user(&user.get_id().to_string()).await {
        Ok(user_fetched) => {
            if user_fetched.is_none() {
                return Html(format!("User not found"));
            } else {
                match update_user(user).await {
                    Ok(user) => {
                        return Html(format!("User updated successfully: {:?}", user));
                    }
                    Err(err) => {
                        return Html(format!("There was an error updating the user: {:?}", err));
                    }
                }
            }
        }
        Err(err) => {
            return Html(format!("There was an error getting the user: {:?}", err));
        }
    };
}

pub async fn delete_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    match delete_user(&id).await {
        Ok(_) => {
            return Html(format!("User with id {} deleted successfully", &id));
        }
        Err(err) => {
            return Html(format!("There was an error deleting the user with id {}: {:?}", &id, err));
        }
    }
}

pub async fn list_users_handler() -> impl IntoResponse {
    match list_users().await {
        Ok(users) => {
            return Html(format!("users: {:?}", users));
        }
        Err(err) => {
            return Html(format!("Something went wrong listing the users: {:?}", err));
        }
    }
}


