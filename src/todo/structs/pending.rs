use super::{base::Base, traits::{create::Create, delete::Delete, edit::Edit, get::Get}};

pub struct Pending {
    super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Pending {
        let super_struct = Base::new(title, "Pending");
        Pending { super_struct }
    }
    pub fn get_status(&self) -> String{
        self.super_struct.get_status()       
    }
    pub fn get_title(&self) -> String{
        self.super_struct.get_title()
    }
}
impl Create for Pending{
    // fn create(&self, title:&str) {
        
    // }
}
impl Edit for Pending{
    // fn set_to_done(&self, title: &str) {
        
    // }
}
impl Get for Pending {
    // fn get(&self,title: &str) {
        
    // }
}
impl Delete for Pending{
    // fn delete(&self, title: &str) {
        
    // }
}