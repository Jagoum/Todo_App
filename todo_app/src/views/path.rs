pub struct Path {
    pub prefix: String,
}
impl Path {
    pub fn define(&self, following_path: String) -> String {
        return self.prefix.to_owned() + &following_path;
    }
}
