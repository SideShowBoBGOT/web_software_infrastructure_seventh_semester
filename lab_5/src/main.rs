use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/", "./static")
                .index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}