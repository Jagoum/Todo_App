pub struct Base {
    title: String,
    status: String,
}
impl Base {
    pub fn new(title: &str, status: &str) -> Base {
        Base {
            title: title.to_string(),
            status: status.to_string(),
        }
    }
    pub fn get_status(&self) -> String {
        self.status.clone()
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
