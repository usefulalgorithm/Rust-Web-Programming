use serde::Serialize;

use super::super::enums::TaskStatus;

#[derive(Debug, Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
