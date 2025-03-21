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
    ///  creates  pending instance
    pub fn new(id: u32, title: &str) -> Pending {
        let super_struct = Base::new(id,title, "Pending");
        Pending { super_struct }
    }
    /// This gives the status of the Pending Job
    pub fn _get_status(&self) -> String {
        self.super_struct.get_status()
    }
    /// THis function gives the title of the Pending Job
    pub fn get_title(&self) -> String {
        self.super_struct.get_title()
    }
    pub fn get_id(&self) -> u32{
        self.super_struct.get_id()
    }
}

impl Edit for Pending {} // It is also possible to change a pending Job to other states like todo and done
impl Delete for Pending {} // It is possible to delete a Job when it is pending
impl Get for Pending {} // It is Possible to Get the status of a job that is pending
