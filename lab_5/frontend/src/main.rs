use actix_web::{App, HttpServer, web};
use actix_files;
use dotenv::dotenv;
use std::env;

async fn serve_students() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/students.html")?)
}

async fn serve_groups() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/groups.html")?)
}

async fn serve_update_student() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/update_student.html")?)
}

async fn serve_update_group() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/update_group.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("FRONTEND_PORT").expect("FRONTEND_PORT must be set");
    let host = env::var("FRONTEND_HOST").expect("FRONTEND_HOST must be set");
    let bind_address = format!("{}:{}", host, port);

    println!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/static", "./static"))
            .route("/", web::get().to(serve_students))
            .route("/students", web::get().to(serve_students))
            .route("/groups", web::get().to(serve_groups))
            .route("/update-student", web::get().to(serve_update_student))
            .route("/update-group", web::get().to(serve_update_group))
    })
        .bind(bind_address)?
        .run()
        .await
}