use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/", web::get().to(root)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

async fn root() -> String {
    "Server is up and running".to_string()
}
