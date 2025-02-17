use crate::to_do::enums::TaskStatus;

use super::base::Base;
#[derive(Debug)]
pub struct Pending {
    pub super_struct: Base,
}
impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        Self { super_struct: base }
    }
}
