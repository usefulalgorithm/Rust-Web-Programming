use actix_web::Responder;

use crate::json_serialization::to_do_items::ToDoItems;

#[actix_web::get("/get")]
pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}
