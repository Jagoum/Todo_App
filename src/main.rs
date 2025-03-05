mod state;
mod todo;
mod process;
use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use todo::todo_factory;
use process::process_input;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> =
    read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
    Some(result) => {
    status = result.to_string().replace('\"', "");
    }
    None=> {
    status = "pending".to_string();
    }
    }
    let item = todo_factory(&status,
    title).expect(&status);
    process_input(item, command.to_string(), &state);
    /*
     println!("Hello, world!");
    let done = Done::new("Shopping");
    let _todo_item = match todo_factory(&done.get_status(), &done.get_title()) {
        Ok(data) => match data {
            ItemTypes::Done(item) => {
                println!("It is a done item with title {}", item.get_title())
            }
            ItemTypes::Pending(item) => {
                println!("It is a pending item with title {}", item.get_title())
            }
        },
        Err(e) => {
            eprintln!("Sorry could get item type {}",e);
            return;
        }
    };
     
     println!("{}", done.get_title());
    // println!("{}\n", done.get_status());
    // let pending = Pending::new("Laundry");
    // println!("{}", pending.get_title());
    // println!("{}", pending.get_status());
*/


}
