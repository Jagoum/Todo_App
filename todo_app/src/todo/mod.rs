pub mod structs;
use structs::done::Done;
use structs::pending::Pending;
use structs::todo::Todo;

pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
    Todo(Todo),
}
/// > A factory is an object for creating other objects
/// So this will be used to create an instance of the state of the different items based on the state entered
pub fn todo_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    if item_type.to_lowercase() == "pending" {
        let item = Pending::new( item_title);
        Ok(ItemTypes::Pending(item))
    } else if item_type.to_lowercase() == "done" {
        let item = Done::new( item_title);
        Ok(ItemTypes::Done(item))
    } else if item_type.to_lowercase() == "todo" {
        let item = Todo::new( item_title);
        Ok(ItemTypes::Todo(item))
    } else {
        Err("This Item Status is not accepted")
    }
}
