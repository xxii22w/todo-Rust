use std::{
    env::{self, VarError},
    sync::Arc,
};

use argonautica::Verifier;

use dotenvy::dotenv;
use sqlx::MySqlPool;
use thiserror::Error;

use crate::{
    db::{DbConnectionPoolError, models::User},
    handlers::auth::models::{LoginRequest, RegistrationRequest},
    service,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool init error: {0}")]
    ConnectionPool(#[from] DbConnectionPoolError),
    #[error("Hashing error: {0}")]
    Hashing(argonautica::Error),
    #[error("Failed to get environment variable: {0}")]
    EnvVar(#[from] VarError),
    #[error("Username already exists: {0}")]
    UsernameAlreadyExists(String),
    #[error("Invalid password")]
    InvalidPassword,
    #[error("JWT service error: {0}")]
    JwtService(#[from] service::jwt::Error),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct Service {
    jwt_service: Arc<service::jwt::Service>,
    conn_pool: MySqlPool,
}

impl Service {
    pub fn new(
        jwt_service: Arc<service::jwt::Service>,
        conn_pool: MySqlPool,
    ) -> Result<Self, Error> {
        Ok(Self {
            jwt_service,
            conn_pool,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<String, Error> {
        dotenv().ok();

        let hashing_secret_key = env::var("HASHING_SECRET_KEY")?;

        let found_user_sql =
            "SELECT id, username, password, created, updated FROM users WHERE username = ?";
        let found_user = sqlx::query_as::<_, User>(found_user_sql)
            .bind(&request.username)
            .fetch_one(&self.conn_pool)
            .await?;

        let mut password_hash_verifier = Verifier::default();
        let pass_valid = password_hash_verifier
            .with_hash(found_user.password.as_str())
            .with_password(request.password)
            .with_secret_key(hashing_secret_key)
            .verify()
            .map_err(|error| Error::Hashing(error))?;

        if !pass_valid {
            return Err(Error::InvalidPassword);
        }

        let token = self.jwt_service.generate_token(&found_user)?;
        Ok(token)
    }

    pub async fn register(&self, request: RegistrationRequest) -> Result<(), Error> {
        dotenv().ok();

        let hashing_secret_key = env::var("HASHING_SECRET_KEY")?;
        let mut hasher = argonautica::Hasher::default();
        let password_hash = hasher
            .with_password(request.password)
            .with_secret_key(hashing_secret_key)
            .hash()
            .map_err(|error| Error::Hashing(error))?;

        let mut tx = self.conn_pool.begin().await?;

        let count_sql = "SELECT COUNT(*) FROM users WHERE username = ?";
        let matching_user_count: i64 = sqlx::query_scalar(count_sql)
            .bind(&request.username)
            .fetch_one(&mut *tx)
            .await?;

        if matching_user_count > 0 {
            return Err(Error::UsernameAlreadyExists(request.username));
        }

        let insert_sql = "INSERT INTO users (username, password) VALUES (?,?)";
        sqlx::query(insert_sql)
            .bind(&request.username)
            .bind(&password_hash)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
