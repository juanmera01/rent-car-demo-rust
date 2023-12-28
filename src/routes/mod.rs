use axum::Router;
use axum::routing::{get, post, put, delete};
use crate::handlers::user_handlers::{create_user_handler, delete_user_handler, get_user_handler, list_users_handler, update_user_handler};
use crate::handlers::vehicle_handlers::{create_vehicle_handler, delete_vehicle_handler, get_vehicle_handler, list_vehicles_handler, update_vehicle_handler};
use crate::handlers::authetication_handlers::{login_handler, signup_handler, index_handler, logout_handler};

pub fn main_router() -> Router {
    Router::new()
        .nest("/",login_routes())
        .nest("/",vehicle_routes())
        .nest("/",user_routes())
        .nest("/",default_routes())
}

// --------------------------

fn login_routes() -> Router {
    Router::new()
        .route("/login",get(login_handler))
        .route("/signup",get(signup_handler))
        .route("/logout",get(logout_handler))
}

fn vehicle_routes() -> Router {
    Router::new()
        .route("/vehicle",get(list_vehicles_handler))
        .route("/vehicle/:id",get(get_vehicle_handler))
        .route("/vehicle",post(create_vehicle_handler))
        .route("/vehicle/:id",delete(delete_vehicle_handler))
        .route("/vehicle",put(update_vehicle_handler))
}

fn user_routes() -> Router {
    Router::new()
        .route("/user",get(list_users_handler))
        .route("/user/:id",get(get_user_handler))
        .route("/user",post(create_user_handler))
        .route("/user/:id",delete(delete_user_handler))
        .route("/user",put(update_user_handler))
}

fn default_routes() -> Router {
    Router::new()
        .route("/",get(index_handler))
}