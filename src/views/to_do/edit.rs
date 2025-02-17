use crate::{database::DB, diesel};
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    schema::to_do,
};

#[actix_web::post("/edit")]
pub async fn edit(to_do_item: web::Json<ToDoItem>, _token: JwToken, mut db: DB) -> HttpResponse {
    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));
    let _ = diesel::update(results).set(to_do::columns::status.eq("DONE")).execute(&mut db.connection);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
