mod api;
mod config;
mod controllers;
mod middlewares;
mod models;
mod path;
mod repositories;
mod router;
mod server;
mod services;
mod utils;

use chat_app_api::router::generate_routes;
use chat_app_api::{
    config::{
        bootstrap::validate_config,
        database::{get_database_pool, run_migration},
        environment::EnvironmentConfig,
        logger::init_logger,
    },
    server::start_server,
};

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvironmentConfig::from_env();
    let _guard = init_logger();

    validate_environment_configs(&config);

    let db_pool = get_database_pool(&config).await?;

    match run_migration(&db_pool).await {
        Ok(_) => {
            tracing::info!("Database migration successful");
        }
        Err(e) => {
            tracing::error!("Database migration failed: {}", e);
            std::process::exit(1);
        }
    }

    let api = generate_routes(&config, &db_pool).await;

    start_server(config.port, api).await?;

    Ok(())
}

fn validate_environment_configs(config: &EnvironmentConfig) {
    if let Err(e) = validate_config(&config) {
        eprintln!("Configuration validation failed: {}", e);
        tracing::error!("Configuration validation failed: {}", e);
        std::process::exit(1);
    }
}
