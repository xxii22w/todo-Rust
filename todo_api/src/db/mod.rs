use std::env::{self, VarError};

use anyhow::Result;

use dotenvy::dotenv;
use sqlx::MySqlPool;
use thiserror::Error;

pub mod models;

#[derive(Error, Debug)]
pub enum DbConnectionPoolError {
    #[error("Missing environment variable: {0}")]
    EnvVar(#[from] VarError),
    #[error("R2D2 DB pool build error: {0}")]
    R2D2(#[from] sqlx::Error),
}

pub async fn connection_pool() -> Result<MySqlPool, DbConnectionPoolError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;

    Ok(pool)
}
