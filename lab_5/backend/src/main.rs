use actix_web::{web, App, HttpServer};
use mongodb::Client as MongoClient;
use std::env;
use sqlx::postgres::PgPoolOptions;

mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Group {
    id: i32,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let pg_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    let pg_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let pg_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let pg_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

    let mongo_host = env::var("MONGO_HOST").expect("MONGO_HOST must be set");
    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT must be set");
    let mongo_db = env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set");
    let mongo_collection = env::var("MONGO_COLLECTION").expect("MONGO_COLLECTION must be set");

    let backend_port = env::var("BACKEND_PORT")
        .expect("BACKEND_PORT must be set")
        .parse::<u16>()
        .expect("BACKEND_PORT must be a valid port number");

    let database_url = format!("postgresql://{pg_user}:{pg_password}@{pg_host}:{pg_port}/{pg_db}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(50))
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let mongo_uri = format!("mongodb://{}:{}", mongo_host, mongo_port);
    let mongo_client = MongoClient::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let mongo_db = mongo_client.database(&mongo_db);
    let mongo_collection = mongo_db.collection::<Group>(&mongo_collection);

    println!("Starting server on port {}", backend_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(mongo_collection.clone()))
            .configure(routes::configure_routes)
    })
        .bind(("0.0.0.0", backend_port))?
        .run()
        .await
}