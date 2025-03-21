use serde_json::value::Value;
use serde_json::Map;
pub trait Get {
    /// This function uses the title which is an id in the map  to retrive the task that the user is looking
    /// If it is not found in the Map, it means it was not present in the Map
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}\n", title);
                println!("Status: {}\n\n", result);
            }
            None => println!("Item: {} was not found", title),
        }
    }
}
