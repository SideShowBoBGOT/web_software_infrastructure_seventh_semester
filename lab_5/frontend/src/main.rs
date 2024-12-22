use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("FRONTEND_PORT").expect("FRONTEND_PORT must be set");
    let host = env::var("FRONTEND_HOST").expect("FRONTEND_HOST must be set");
    let bind_address = format!("{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/", "./static")
                .index_file("index.html"))
    })
        .bind(bind_address)?
        .run()
        .await
}