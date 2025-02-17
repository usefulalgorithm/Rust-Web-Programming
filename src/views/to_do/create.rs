use actix_web::{web::Path, HttpResponse};
use crate::{diesel, models::item::new_item::NewItem};
use diesel::prelude::*;

use crate::{
    database::establish_connection,
    json_serialization::to_do_items::ToDoItems,
    models::item::item::Item,
    schema::to_do,
};

#[actix_web::post("/create/{title}")]
pub async fn create(path: Path<String>) -> HttpResponse {
    let title = path.into_inner();
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table).values(&new_post).execute(&mut connection);
    }
    HttpResponse::Ok().json(ToDoItems::get_state())
}
