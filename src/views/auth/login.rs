use actix_web::{web::Json, HttpResponse};
use diesel::prelude::*;

use crate::{
    database::DB,
    json_serialization::{login::Login, login_response::LoginResponse},
    jwt::JwToken,
    models::user::user::User,
    schema::users,
};

pub async fn login(credentials: Json<Login>, mut db: DB) -> HttpResponse {
    let loaded_users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&mut db.connection)
        .unwrap();
    match &loaded_users[..] {
        [] => HttpResponse::NotFound().await.unwrap(),
        [user] => match user.verify(credentials.password.clone()) {
            true => {
                let raw_token = JwToken::new(user.id).encode();
                HttpResponse::Ok()
                    .append_header(("token", raw_token.clone()))
                    .json(LoginResponse { token: raw_token })
            }
            false => HttpResponse::Unauthorized().finish(),
        },
        _ => HttpResponse::Conflict().await.unwrap(),
    }
}
