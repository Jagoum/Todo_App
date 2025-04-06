

use crate::todo::structs::base::Base;
pub struct ToDoItems {
pub pending_items: Vec<Base>,
pub done_items: Vec<Base>,
pub pending_item_count: i8,
pub done_item_count: i8
}

impl serde::Serialize for ToDoItems {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}