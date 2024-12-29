mod config;

use config::{database::MySqlDatabase, environment::Config};
use dotenv::dotenv;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let config = Config::from_env();

    let database = MySqlDatabase::new(config.database_url).await;

    database.run_migration().await?;

    println!("Connected to database");

    Ok(())
}
