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

#[cfg(test)]
mod jwt_tests {
    use actix_service::Service;
    use actix_web::{http::{header::ContentType, StatusCode}, test::{call_and_read_body_json, call_service, init_service, TestRequest}, web::get, App, HttpRequest, HttpResponse};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::{config::Config, jwt::JwToken};

    #[test]
    fn get_key() {
        assert_eq!("secret".to_string(), JwToken::get_key());
    }

    #[test]
    fn get_exp() {
        assert_eq!(Config::new().expire_minutes, 120);
    }

    #[test]
    fn decode_incorrect_token() {
        match JwToken::from_token("invalid token".to_string()) {
            Err(message) => assert_eq!(message, "InvalidToken".to_string()),
            _ => assert!(false, "should not decode invalid token"),
        }
    }

    #[test]
    fn encode_decode() {
        let tok = JwToken::new(1);
        let encoded = tok.encode();
        match JwToken::from_token(encoded) {
            Ok(decoded) => assert_eq!(decoded.user_id, 1),
            _ => assert!(false, "Should be able to decode valid token"),
        }
    }
    async fn test_handler(token: JwToken, _: HttpRequest) -> HttpResponse {
        HttpResponse::Ok().json(json!({
            "user_id": token.user_id,
            "exp_minutes": 60,
        }))
    }
    #[actix_web::test]
    async fn test_no_token_request() {
        let app = init_service(App::new().route("/", get().to(test_handler))).await;
        let req = TestRequest::default().insert_header(ContentType::plaintext()).to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32
    }

    #[actix_web::test]
    async fn test_passing_token_request() {
        let token = JwToken::new(123).encode();
        let app = init_service(App::new().route("/", get().to(test_handler))).await;
        let req = TestRequest::default().append_header(ContentType::plaintext()).append_header(("token".to_string(), token)).to_request();
        let resp: ResponseFromTest = call_and_read_body_json(&app, req).await;
        assert_eq!(resp.user_id, 123);
    }

    #[actix_web::test]
    async fn test_false_token_request() {
        let app = init_service(App::new().route("/", get().to(test_handler))).await;
        let req = TestRequest::default().append_header(ContentType::plaintext()).append_header(("token".to_string(), "not a token".to_string())).to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}