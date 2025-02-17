use crate::{database::DB, diesel, models::item::item::Item, schema::to_do};
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
};

#[actix_web::post("/delete")]
pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken, mut db: DB) -> HttpResponse {
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .filter(to_do::columns::user_id.eq(token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut db.connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut db.connection);
    HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}
