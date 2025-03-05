use std::iter;

use todo::{structs::{done::Done, pending::Pending}, todo_factory, ItemTypes};

mod todo;

fn main() {
    // println!("Hello, world!");

    let done = Done::new("Shopping");
    let todo_item = match todo_factory(&done.get_status(), &done.get_title()) {
        Ok(data) => {match data {
        ItemTypes::Done(item   ) => {println!("It is a done item with title {}",item.get_title())},
        ItemTypes::Pending(item)=> {println!("It is a pending item with title {}", item.get_title())},
        }},
        Err(e) => {eprintln!("Sorry could get item type");return;},
    };
    println!("{}",done.get_title());
    println!("{}\n",done.get_status());

    let pending = Pending::new("Laundry");
    println!("{}",pending.get_title());
    println!("{}",pending.get_status());

}
