use todo::structs::{done::Done, pending::Pending};

mod todo;

fn main() {
    // println!("Hello, world!");
    let done = Done::new("Shopping");
    println!("{}",done.get_title());
    println!("{}\n",done.get_status());

    let pending = Pending::new("Laundry");
    println!("{}",pending.get_title());
    println!("{}",pending.get_status());

}
