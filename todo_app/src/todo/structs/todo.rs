use super::{
    base::Base,
    traits::{create::Create, delete::Delete, edit::Edit, get::Get},
};
/// This struct is to create a job or switch a job with the status of todo
pub struct Todo {
    super_struct: Base,
}

impl Todo {
    pub fn new(title: &str) -> Todo {
        let super_struct = Base::new(title, "Todo");
        Todo { super_struct }
    }
    pub fn get_status(&self) -> String {
        self.super_struct.get_status()
    }
    pub fn get_title(&self) -> String {
        self.super_struct.get_title()
    }
}

impl Create for Todo {}
impl Edit for Todo {}
impl Delete for Todo {}
impl Get for Todo {}
