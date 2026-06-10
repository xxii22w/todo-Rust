use std::{
    env::{self, VarError},
    sync::Arc,
};

use argonautica::Verifier;
use diesel::{
    Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl, SelectableHelper,
    r2d2::{self, ConnectionManager, Pool},
};
use dotenvy::dotenv;
use thiserror::Error;

use crate::{
    db::{
        DbConnectionPoolError,
        models::{CreateUser, User},
        schema,
    },
    handlers::auth::models::{LoginRequest, RegistrationRequest},
    service,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool init error: {0}")]
    ConnectionPool(#[from] DbConnectionPoolError),
    #[error("R2D2 DB pool build error: {0}")]
    R2D2(#[from] r2d2::PoolError),
    #[error("DB error: {0}")]
    Diesel(#[from] diesel::result::Error),
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
}

pub struct Service {
    jwt_service: Arc<service::jwt::Service>,
    conn_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Service {
    pub fn new(
        jwt_service: Arc<service::jwt::Service>,
        conn_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            jwt_service,
            conn_pool,
        })
    }

    pub fn login(&self, request: LoginRequest) -> Result<String, Error> {
        dotenv().ok();

        let hashing_secret_key = env::var("HASHING_SECRET_KEY")?;
        let mut conn = self.conn_pool.get()?;
        let found_user: User = schema::users::table
            .filter(schema::users::username.eq(request.username))
            .select(User::as_select())
            .first(&mut conn)?;

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

    pub fn register(&self, request: RegistrationRequest) -> Result<(), Error> {
        dotenv().ok();

        let hashing_secret_key = env::var("HASHING_SECRET_KEY")?;
        let mut hasher = argonautica::Hasher::default();
        let password_hash = hasher
            .with_password(request.password)
            .with_secret_key(hashing_secret_key)
            .hash()
            .map_err(|error| Error::Hashing(error))?;

        let mut conn = self.conn_pool.get()?;
        conn.transaction(|conn| {
            let matching_username_count: i64 = schema::users::table
                .filter(schema::users::username.eq(request.username.as_str()))
                .count()
                .get_result(conn)?;

            if matching_username_count > 0 {
                return Err(Error::UsernameAlreadyExists(request.username));
            }

            diesel::insert_into(schema::users::table)
                .values(&CreateUser {
                    username: request.username,
                    password: password_hash,
                })
                .execute(conn)?;

            Ok::<(), Error>(())
        })?;

        Ok(())
    }
}
