use super::{
    base::Base,
    traits::{delete::Delete, edit::Edit, get::Get},
};

pub struct Done {
    super_stuct: Base,
}

impl Done {
    pub fn new(title: &str) -> Done {
        let base = Base::new(title, "Done");
        Done { super_stuct: base }
    }
    pub fn _get_status(&self) -> String {
        self.super_stuct.get_status()
    }
    /// Using this method is to avoith the fields of my struct to be accessed directly
    pub fn get_title(&self) -> String {
        self.super_stuct.get_title()
    }
}

impl Get for Done {
    /*
     fn get(&self,titile: &str) {

    // }  // this functions have already been implemented in the Get trait so there is no need to implement it again
    // This function was implemented directly in the trait because all the structs that call the function will implement it thesame way it is .
    // So implementing Directly in the trait reduces code duplication */
}
impl Delete for Done {} // delete fn has already been implemented in the trait Delete
impl Edit for Done {} // edit funtion for both done and pending has already been implemented in the Edit trait
