use super::{base::Base, traits::{delete::Delete, edit::Edit, get::Get}};

pub struct Done{
    super_stuct: Base
}

impl Done {
    pub fn  new( title:&str) -> Done{
        let base = Base::new(title, "done");
        Done { super_stuct: base }
    }
    pub fn get_status(&self) -> String {
        self.super_stuct.get_status()
    }
    pub fn get_title(&self) -> String{
        self.super_stuct.get_title()
    }
}

impl Get for Done{
    // fn get(&self,titile: &str) {
        
    // }
}
impl Delete for Done{
    // fn delete(&self, title: &str) {
        
    // }
}
impl Edit for Done{
    // fn set_to_pending(&self, title: &str) {
        
    // }
    // fn set_to_done(&self, title: &str) {
        
    // } // this method have already been implemented for the with delcarations
}