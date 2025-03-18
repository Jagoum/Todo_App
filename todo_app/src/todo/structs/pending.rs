use super::{
    base::Base,
    traits::{delete::Delete, edit::Edit, get::Get},
};

/// This struct is to set the state to pending
///  It also implements methods that can allow the struct to be edited, accessed and deleted
pub struct Pending {
    super_struct: Base,
}

impl Pending {
    /// Sets the instance created to pending
    pub fn new(title: &str) -> Pending {
        let super_struct = Base::new(title, "Pending");
        Pending { super_struct }
    }
    pub fn get_status(&self) -> String {
        self.super_struct.get_status()
    }
    pub fn get_title(&self) -> String {
        self.super_struct.get_title()
    }
}

impl Edit for Pending {}
impl Delete for Pending {}
impl Get for Pending {}
