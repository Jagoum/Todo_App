use crate::state::write_to_file;
use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
pub trait Edit {
    /// This function changes the status from what it was to Done
    ///  > Example
    ///
    /// ```
    /// New Job is being set to Done
    ///
    /// Status: "Done"
    /// ```    
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        
        state.insert(title.to_string(), json!(String::from("Done")));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to done\n", title);
        println!(
            "Status: {}\n",
            state.get(title).unwrap_or(&json!(String::from("Done")))
        );
    }


    /// This function changes the status from what it was to pending
    ///  > Example
    ///
    /// ```
    /// New Job is being set to Pending
    ///
    /// Status: "Pending"
    /// ```
    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        // This is used to insert a new value at a particular key . It is just like setting and element at a particular index or modifying the element at that index
        state.insert(title.to_string(), json!(String::from("Pending")));
        write_to_file("./state.json", state); // This then writes the new state to the file overriding the previous one

        println!("\n\n{} is being  set to pending\n", title);
        println!(
            "Status: {}\n",
            state.get(title).unwrap_or(&json!(String::from("Pending")))
        );
    }

    /// This function changes the status from what it was to Todo
    ///  > Example
    ///
    /// ```
    /// New Job is being set to Todo
    ///
    /// Status: "Todo"
    /// ```
    fn set_to_todo(&self, title: &String, state: &mut Map<String, Value>) {

        state.insert(title.to_string(), json!(String::from("Todo")));
        write_to_file("./state.json", state);

        println!("\n\n{} is being set to todo\n", title);
        println!(
            "Status: {}\n",
            state.get(title).unwrap_or(&json!(String::from("Todo")))
        )
    }
}
