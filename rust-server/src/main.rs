mod config;
mod error;
mod handlers;
mod models;
mod services;
mod utils;
mod templates;

use actix_web::{middleware,web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    // Load configuration
    let config = config::Config::from_env().expect("Server configuration");
    
    // Ensure upload directory exists
    services::storage::ensure_upload_dir(&config.upload_dir)?;
    
    info!("Starting server at http://localhost:{}", config.port);
    info!("Upload directory: {}", config.upload_dir);
    let config_copy: config::Config = config.clone();
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(config.clone()))
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Limit the maximum size of an uploaded file
            .wrap(middleware::Compress::default())
            // Register services
            .configure(handlers::config)
    })
    .bind(format!("0.0.0.0:{}", config_copy.port))?
    .run()
    .await
}