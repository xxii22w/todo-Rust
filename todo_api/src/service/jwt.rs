use std::{
    collections::BTreeMap,
    env::{self, VarError},
    num::ParseIntError,
};

use dotenvy::dotenv;
use hmac::{Hmac, Mac, digest::InvalidLength};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use thiserror::Error;

use crate::db::models::User;

pub struct Service {
    jwt_secret: Hmac<Sha256>,
}

#[derive(Clone)]
pub struct ContextUser {
    pub user_id: u64,
    pub username: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing environment variable: {0}")]
    EvnVar(#[from] VarError),
    #[error("HMAC invalid length: {0}")]
    InvalidHmacLength(#[from] InvalidLength),
    #[error("JWT error: {0}")]
    JWT(#[from] jwt::Error),
    #[error("Missing user ID in JWT token")]
    JwtUserIdMissing,
    #[error("Missing username in JWT token")]
    JwtUsernameMissing,
    #[error("Failed to parse int from string: {0}")]
    ParseInt(#[from] ParseIntError),
}

impl Service {
    pub fn new() -> Result<Self, Error> {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET")?;
        Ok(Self {
            jwt_secret: Hmac::new_from_slice(jwt_secret.as_bytes())?,
        })
    }

    pub fn generate_token(&self, user: &User) -> Result<String, Error> {
        let mut claims = BTreeMap::new();
        let user_id_str = user.id.to_string();
        claims.insert("sub", user_id_str.as_str());
        claims.insert("name", user.username.as_str());
        let token = claims.sign_with_key(&self.jwt_secret)?;
        Ok(token)
    }

    pub fn verify_token(&self, token: String) -> Result<ContextUser, Error> {
        let token: Token<Header, BTreeMap<String, String>, _> =
            token.verify_with_key(&self.jwt_secret)?;
        let user_id: u64 = token
            .claims()
            .get("sub")
            .ok_or(Error::JwtUserIdMissing)?
            .parse()?;
        let username = token
            .claims()
            .get("name")
            .ok_or(Error::JwtUsernameMissing)?;
        Ok(ContextUser {
            user_id,
            username: username.clone(),
        })
    }
}
