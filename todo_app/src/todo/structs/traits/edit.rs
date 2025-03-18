use crate::state::write_to_file;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
pub trait Edit {
    /// - This function changes the status from what ever it was probably pending to done
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("done")));
        write_to_file("./state.json", state);
        println!("\n\n{} is being set to done\n", title);
        println!("Status: \"Done\"\n");
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(String::from("pending")));
        write_to_file("./state.json", state);
        println!("\n\n{} is being  set to pending\n", title);
        println!("Status: \"Pending\"\n");
    }

    fn set_to_todo(&self, title: &String, state: &mut Map<String, Value>) {}
}
