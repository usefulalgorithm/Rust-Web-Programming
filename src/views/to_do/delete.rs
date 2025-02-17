use crate::{diesel, models::item::item::Item, schema::to_do};
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use crate::{
    database::establish_connection,
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
};

#[actix_web::post("/delete")]
pub async fn delete(to_do_item: web::Json<ToDoItem>, _token: JwToken) -> HttpResponse {
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut connection);
    HttpResponse::Ok().json(ToDoItems::get_state())
}
