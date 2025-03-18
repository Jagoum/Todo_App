use crate::state::write_to_file;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
pub trait Create {
    /// > This function takes is used to handle the command create.
    /// > It takes the title with is the id of the Map (key) and the status is the value of the map and can either be pending or done
    /// > The state is  a map that determines the state of current takes or that stores the state of our current task as json
    ///  See ./state/write_to_file for how it works.
    /// > But briefly write_to_file just write the json file in our map into the state.json file
    /// > The function then prints the line to tell the user that task was created
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file("./state.json", state);
        println!("\n\n{} is being created\n\n", title);
        println!("Status: \"{}\"\n\n ", status);
    }
}
