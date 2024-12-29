use sqlx::{migrate::Migrator, MySql, MySqlPool, Pool};

pub struct MySqlDatabase {
    pub database_url: String,
    pub pool: Pool<MySql>,
}

impl MySqlDatabase {
    pub async fn new(database_url: String) -> Self {
        let pool = get_database_pool(&database_url).await;
        Self { database_url, pool }
    }

    pub async fn run_migration(&self) -> Result<(), sqlx::Error> {
        let migrator = Migrator::new(std::path::Path::new("migrations"))
            .await
            .expect("Failed to create migrator");

        migrator.run(&self.pool).await?;

        Ok(())
    }
}

pub async fn get_database_pool(url: &String) -> Pool<MySql> {
    MySqlPool::connect(&url)
        .await
        .expect("Failed to connect to database")
}
