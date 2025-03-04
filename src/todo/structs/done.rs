use super::base::Base;

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