use actix_web::error::{Error, ErrorUnauthorized};
use actix_web::FromRequest;
use chrono::Utc;
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Deserialize, Serialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn get_key() -> String {
        Config::new().secret_key
    }
    pub fn encode(self) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(JwToken::get_key().as_ref()),
        )
        .unwrap()
    }
    pub fn new(user_id: i32) -> Self {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(Config::new().expire_minutes))
            .unwrap()
            .timestamp();
        Self {
            user_id,
            exp: exp as usize,
        }
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        match decode::<JwToken>(&token, &key, &Validation::new(Algorithm::HS256)) {
            Ok(data) => Ok(data.claims),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl FromRequest for JwToken {
    type Error = Error;

    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                match Self::from_token(raw_token) {
                    Ok(token) => ok(token),
                    Err(error) => err(ErrorUnauthorized(error)),
                }
            }
            None => err(ErrorUnauthorized("No token found in header")),
        }
    }
}
