use crate::todo::structs::todo::Todo;

use super::todo::structs::done::Done;
use super::todo::structs::pending::Pending;
use super::todo::structs::traits::create::Create;
use super::todo::structs::traits::delete::Delete;
use super::todo::structs::traits::edit::Edit;
use super::todo::structs::traits::get::Get;
use super::todo::ItemTypes;
use serde_json::value::Value;
use serde_json::Map;
/// This one performs the given command on pending status
/// Any of this command command here will have the status pending except when
/// ## example of commands
/// ```rs
/// cargo run create NewJob
/// cargo run edit NewJob
/// cargo run get NewJob
/// cargo run delete NewJob
///
/// ```
/// > Function take 3 parameters `item`, `command`, and `state`
fn process_pending(
    item: Pending,
    command: String,
    state: &Map<String, Value>,
    target_state: String,
) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.get_title(), &state),
        "delete" => item.delete(&item.get_title(), &mut state),
        "edit" => match target_state.to_lowercase().as_str() {
            "done" => item.set_to_done(&item.get_title(), &mut state),
            "todo" => item.set_to_todo(&item.get_title(), &mut state),
            "pending" => println!(
                "Cannot change state from {} to {}",
                target_state, target_state
            ),
            _ => println!("Invalid State {}", target_state),
        },
        _ => println!("command: {} not supported", command),
    }
}
///This function processes done for the repective commands
/// ## example of commands
/// ```rs
///
/// cargo run edit NewJob
/// cargo run get NewJob
/// cargo run delete NewJob
///
/// ```
/// > Function take 4 parameters `item`, `command`, and `state` and `target state`
fn process_done(item: Done, command: String, state: &Map<String, Value>, target_state: String) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.get_title(), &state),
        "delete" => item.delete(&item.get_title(), &mut state),

        "edit" => match target_state.to_lowercase().as_str() {
            "todo" => item.set_to_todo(&item.get_title(), &mut state),
            "pending" => item.set_to_pending(&item.get_title(), &mut state),
            "done" => println!(
                "Cannot change state from {} to {}",
                target_state, target_state
            ),
            _ => println!("Invalid State {}", target_state),
        },

        _ => println!("command: {} not supported", command),
    }
}

fn process_todo(item: Todo, command: String, state: &Map<String, Value>, target_state: String) {
    let mut state = state.clone();
    match command.as_str() {
        "create" => item.create(&item.get_title(), &item.get_status(), &mut state),

        "edit" => match target_state.to_lowercase().as_str() {
            "done" => item.set_to_done(&item.get_title(), &mut state),
            "pending" => item.set_to_pending(&item.get_title(), &mut state),
            "todo" => println!(
                "Cannot Change state from {} to {}",
                target_state, target_state
            ),
            _ => println!("Invalid State {} ", target_state),
        },

        "get" => item.get(&item.get_title(), &state),
        "delete" => item.delete(&item.get_title(), &mut state),
        _ => println!("Sorry invalid command: {} not supported", command),
    }
}

///  > This function is used to process the different inputs from the user
///  It gets the inputs and base on the ItemType calls one of either the function `process_pending()` or the function `process_done()` or `process_todo()`
///
pub fn process_input(
    item: ItemTypes,
    command: String,
    state: &Map<String, Value>,
    target_state: String,
) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state, target_state),
        ItemTypes::Done(item) => process_done(item, command, state, target_state),
        ItemTypes::Todo(item) => process_todo(item, command, state, target_state),
    }
}
