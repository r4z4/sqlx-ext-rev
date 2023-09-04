mod hello_world;


use http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};
use axum::{
    middleware,
    routing::{get, post, put, patch, delete},
    Router, body::Body, http::Method, Extension,
};
use hello_world::hello_world;


#[derive(Clone)]
pub struct SharedData {
    pub message: String
}

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let shared_data = SharedData {
        message: "Hello from shared data".to_owned()
    };

    Router::new()
        .route("/hello", get(hello_world))
        .route("/", get(hello_world))
        .layer(cors)
        .layer(Extension(database))
}