use actix_web::Responder;

use crate::{json_serialization::to_do_items::ToDoItems, jwt::JwToken};

#[actix_web::get("/get")]
pub async fn get(token: JwToken) -> impl Responder {
    ToDoItems::get_state(token.user_id)
}
