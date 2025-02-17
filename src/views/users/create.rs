use actix_web::{web, HttpResponse, Responder};
use diesel::RunQueryDsl;

use crate::{
    database::DB, json_serialization::new_user::NewUserSchema, models::user::new_user::NewUser,
    schema::users,
};

#[actix_web::post("/create")]
pub async fn create(new_user: web::Json<NewUserSchema>, mut db: DB) -> impl Responder {
    let new_user = NewUser::new(
        new_user.name.clone(),
        new_user.email.clone(),
        new_user.password.clone(),
    );
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut db.connection)
    {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::Conflict(),
    }
}
