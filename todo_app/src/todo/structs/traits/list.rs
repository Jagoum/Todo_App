/// This trait is to list all jobs with their status
pub trait ListTask {
    fn list_task(&self, file_name: String);
}
