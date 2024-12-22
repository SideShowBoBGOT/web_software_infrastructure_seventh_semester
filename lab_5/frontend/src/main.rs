use actix_web::{App, HttpServer, web};
use actix_files;
use std::env;
use std::fs;

async fn inject_env_and_serve(file_path: &str) -> actix_web::Result<actix_web::HttpResponse> {
    let mut content = fs::read_to_string(file_path)?;

    let backend_host = env::var("BACKEND_HOST").unwrap();
    let backend_port = env::var("BACKEND_PORT").unwrap();

    let config_script = format!(
        r#"<script>
            window.API_BASE_URL = "http://{}:{}";
        </script>"#,
        backend_host, backend_port
    );

    content = content.replace("<head>", &format!("<head>\n    {}", config_script));

    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(content))
}

async fn serve_students() -> actix_web::Result<actix_web::HttpResponse> {
    inject_env_and_serve("./static/students.html").await
}

async fn serve_groups() -> actix_web::Result<actix_web::HttpResponse> {
    inject_env_and_serve("./static/groups.html").await
}

async fn serve_update_student() -> actix_web::Result<actix_web::HttpResponse> {
    inject_env_and_serve("./static/update_student.html").await
}

async fn serve_update_group() -> actix_web::Result<actix_web::HttpResponse> {
    inject_env_and_serve("./static/update_group.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "standalone")]
    {
        let path_env = {
            let current_exe = std::env::current_exe().expect("Failed to get executable path");
            let project_root = current_exe
                .parent() // executable directory
                .and_then(|p| p.parent()) // target directory
                .and_then(|p| p.parent()) // project root
                .expect("Failed to find project root");
            project_root.join("../.env")
        };
        dotenv::from_path(path_env).ok();
    }

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