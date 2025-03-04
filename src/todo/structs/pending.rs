use super::base::Base;

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
