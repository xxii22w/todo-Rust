use std::env::{self, VarError};

use anyhow::Result;
use diesel::{
    MysqlConnection,
    r2d2::{self, ConnectionManager, Pool},
};
use dotenvy::dotenv;
use thiserror::Error;

pub mod migration;
pub mod models;
pub mod schema;

#[derive(Error, Debug)]
pub enum DbConnectionPoolError {
    #[error("Missing environment variable: {0}")]
    EnvVar(#[from] VarError),
    #[error("R2D2 DB pool build error: {0}")]
    R2D2(#[from] r2d2::PoolError),
}

pub fn connection_pool() -> Result<Pool<ConnectionManager<MysqlConnection>>, DbConnectionPoolError>
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Ok(Pool::builder().test_on_check_out(true).build(manager)?)
}
