pub mod structs;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes{
    Done(Done),
    Pending(Pending)
}
pub fn todo_factory(item_type:&str, item_title: &str) -> Result<ItemTypes, &'static str>{
    if item_type.to_lowercase() == "pending"{
        let item = Pending::new(item_title);
        Ok(ItemTypes::Pending(item))
    }
    else if  item_type.to_lowercase() == "done" {
        let item = Done::new(item_title);
        Ok(ItemTypes::Done(item))
    }
    else{
        Err("This Item Status is not accepted")
    }
}