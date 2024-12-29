use sqlx::{migrate::Migrator, MySql, MySqlPool, Pool};

use super::environment::EnvironmentConfig;

pub async fn get_database_pool(config: &EnvironmentConfig) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPool::connect(&config.database_url).await
}

pub async fn run_migration(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    const MIGRATIONS_DIR: &str = "migrations";

    let directory = std::path::Path::new(MIGRATIONS_DIR);
    let migrator = Migrator::new(directory).await?;

    migrator.run(pool).await?;

    Ok(())
}
