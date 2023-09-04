use actix_web::{web, App, HttpServer, http::header::ContentType, HttpResponse};
use sqlx::postgres::{ PgPool, PgPoolOptions, PgRow };
use sqlx::*;
use dotenvy_macro::dotenv;
use std::env;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let pg_uri: &'static str = dotenv!("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_uri).await.unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || 
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .route("/get/{user_id}", web::get().to(get_user)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn root() -> String {
    "Server is up and running".to_string()
}

async fn get_user(path: web::Path<usize>, app_state: web::Data::<AppState>) -> HttpResponse {
    let user_id: usize = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct User {
        user_id: i32,
        username: String,
    }

    let user: sqlx::Result<Option<User>> = sqlx::query_as!(
        User,
        "SELECT user_id, username FROM users WHERE user_id = 1;"
    ).fetch_optional(&app_state.pool).await;

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("X-Hdr", "sample"))
        .body("data")
}