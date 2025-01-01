use sqlx::{MySql, Pool};

use super::environment::EnvironmentConfig;

#[derive(Clone, Debug)]
pub struct ApiState {
    pub db: Pool<MySql>,
    pub environment: EnvironmentConfig,
}

impl ApiState {
    pub fn new(db: Pool<MySql>, environment: EnvironmentConfig) -> Self {
        Self { db, environment }
    }
}
