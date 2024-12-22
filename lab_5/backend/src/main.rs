use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;
use mongodb::Client as MongoClient;
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    let pg_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let pg_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let pg_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let pg_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let pg_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

    let mongo_host = env::var("MONGO_HOST").unwrap_or_else(|_| "localhost".to_string());
    let mongo_port = env::var("MONGO_PORT").unwrap_or_else(|_| "27017".to_string());
    let mongo_db = env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set");
    let mongo_collection = env::var("MONGO_COLLECTION").expect("MONGO_COLLECTION must be set");

    let backend_port = env::var("BACKEND_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("BACKEND_PORT must be a valid port number");

    // PostgreSQL connection
    let pg_config = format!(
        "host={} port={} user={} password={} dbname={}",
        pg_host, pg_port, pg_user, pg_password, pg_db
    );
    let (pg_client, pg_connection) = tokio_postgres::connect(&pg_config, NoTls)
        .await
        .expect("Failed to connect to PostgreSQL");

    // Spawn PostgreSQL connection handler
    tokio::spawn(async move {
        if let Err(e) = pg_connection.await {
            eprintln!("PostgreSQL connection error: {}", e);
        }
    });

    // MongoDB connection
    let mongo_uri = format!("mongodb://{}:{}", mongo_host, mongo_port);
    let mongo_client = MongoClient::with_uri_str(&mongo_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let mongo_db = mongo_client.database(&mongo_db);
    let mongo_collection = mongo_db.collection(&mongo_collection);

    // Start HTTP server
    println!("Starting server on port {}", backend_port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pg_client))
            .app_data(web::Data::new(mongo_collection.clone()))
            .configure(routes::configure_routes)
    })
        .bind(("0.0.0.0", backend_port))?
        .run()
        .await
}