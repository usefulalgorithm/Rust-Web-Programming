use crate::schema::to_do;
use chrono::{NaiveDateTime, Utc};

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime
}

impl NewItem {
    pub fn new(title: String) -> Self {
        Self {
            title,
            status: String::from("PENDING"),
            date: Utc::now().naive_local()
        }
    }
}