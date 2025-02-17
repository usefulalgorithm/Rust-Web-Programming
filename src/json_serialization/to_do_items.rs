use diesel::prelude::*;
use std::str::FromStr;

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use itertools::Itertools;
use serde::Serialize;
use serde_json::to_string;

use crate::{
    database::DB_CONNECTION,
    models::item::item::Item,
    schema::to_do,
    to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes},
};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let (pending_items, done_items): (Vec<Base>, Vec<Base>) =
            input_items.into_iter().partition_map(|item| match item {
                ItemTypes::Pending(pending) => itertools::Either::Left(pending.super_struct),
                ItemTypes::Done(done) => itertools::Either::Right(done.super_struct),
            });
        let pending_item_count = pending_items.len() as i8;
        let done_item_count = done_items.len() as i8;
        Self {
            pending_items,
            done_items,
            pending_item_count,
            done_item_count,
        }
    }

    pub fn get_state(user_id: i32) -> Self {
        let mut connection = DB_CONNECTION.db_connection.get().unwrap();
        let items = to_do::table
            .filter(to_do::columns::user_id.eq(user_id))
            .order(to_do::columns::id.asc())
            .load::<Item>(&mut connection)
            .unwrap();
        Self::new(
            items
                .iter()
                .map(|item| to_do_factory(&item.title, TaskStatus::from_str(&item.status).unwrap()))
                .collect(),
        )
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
