/// This struct is to initialize an instance of the job with its state
/// This struct is responsible for the new tasks that will be created
///
pub struct Base {
    id: u32,
    title: String,
    status: String,
}
impl Base {
    /// This function creates a new instance with the title and the status
    pub fn new(id: u32, title: &str, status: &str) -> Base {
        Base {
            id,
            title: title.to_string(),
            status: status.to_string(),
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    /// This is a method to get the status of an instance of a new task
    pub fn get_status(&self) -> String {
        self.status.clone()
    }
    /// This method is to get the title of a task that is initiated
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
