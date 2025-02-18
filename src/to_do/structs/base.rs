use serde::Serialize;

use super::super::enums::TaskStatus;

#[derive(Debug, Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}

#[cfg(test)]
mod base_tests {
    use super::TaskStatus;
    use super::Base;

    #[test]
    fn new() {
        let expected_title = "test title".to_string();
        let expected_status = TaskStatus::DONE;
        let base = Base {
            title: expected_title.clone(),
            status: TaskStatus::DONE,
        };
        assert_eq!(expected_title, base.title);
        assert_eq!(expected_status, base.status);
    }
}