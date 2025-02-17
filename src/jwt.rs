use actix_web::{http::Error, FromRequest};
use futures::future::{ok, Ready};

pub struct JwToken {
    pub message: String
}

impl FromRequest for JwToken {
    type Error = Error;

    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let token = JwToken { message: data.to_str().unwrap().to_string() };
                ok(token)
            },
            None => {
                let token = JwToken { message: String::from("Nothing found") };
                ok(token)
            },
        }
    }
}