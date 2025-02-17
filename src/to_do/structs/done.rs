use crate::to_do::enums::TaskStatus;

use super::base::Base;
#[derive(Debug)]
pub struct Done {
    pub super_struct: Base,
}
impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        Self { super_struct: base }
    }
}