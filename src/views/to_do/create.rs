use crate::{database::DB, diesel, jwt::JwToken, models::item::new_item::NewItem};
use actix_web::{web::Path, HttpResponse};
use diesel::prelude::*;

use crate::{json_serialization::to_do_items::ToDoItems, models::item::item::Item, schema::to_do};

#[actix_web::post("/create/{title}")]
pub async fn create(path: Path<String>, token: JwToken, mut db: DB) -> HttpResponse {
    let title = path.into_inner();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut db.connection)
        .unwrap();
    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut db.connection);
    }
    HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}
