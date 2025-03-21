use serde_json::value::Value;
use serde_json::Map;
pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Status: {}\n\n", result);
            }
            None => println!("item: {} was not found", title),
        }
    }
}
