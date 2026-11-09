/* MODs */
mod api;
mod dtos;
mod task;

/* USEs */
use actix_web::{App, HttpServer};
use api::health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
