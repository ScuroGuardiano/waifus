mod db;
mod schema;
mod models;

use std::env;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use crate::db::create_db_pool;

#[derive(Clone)]
struct AppConfig {
    main_domain: String,
    database_url: String,
    host: String,
    port: u16
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config();
    let host = config.host.clone();
    let port = config.port;
    let pool = create_db_pool(&config.database_url);

    let server = HttpServer::new(move || {
       App::new()
           .app_data(pool.clone())
           .app_data(config.clone())
           .service(hello)
    })
    .bind((host.clone(), port))?
    .run();

    println!("Orchestrator is listening on http://{}:{}", host, port);

    server.await
}

fn load_config() -> AppConfig {
    dotenv().ok();

    let port_str = env::var("PORT").unwrap_or("2137".to_owned());
    let port: u16 = port_str.parse().expect("Port has to be valid 16 bit unsigned integer.");
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());

    let config = AppConfig {
        main_domain: env::var("MAIN_DOMAIN").expect("MAIN_DOMAIN environment variable must be set."),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set."),
        host, port
    };

    config
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}