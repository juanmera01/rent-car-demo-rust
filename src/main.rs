use axum::Router;
use tokio::net::TcpListener;
use crate::routes::main_router;

mod routes;
mod handlers;
mod model;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(main_router());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
}
