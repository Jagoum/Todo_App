
use std::env;

use serde_json::json;
use state::{read_file, write_to_file};
use todo::{
    structs::{done::Done, pending::Pending},
    todo_factory, ItemTypes,
};
mod state;
mod todo;

fn main() {
    let args : Vec<String> = env::args().collect();
    let status = &args[1];
    let title = &args[2];

    let mut state = read_file("./state.json");
    println!("{:#?}",state);
    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);
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
