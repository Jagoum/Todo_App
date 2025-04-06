mod process;
mod state;
mod todo;
mod views;
use actix_web::{App, HttpServer};
use process::process_input;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;
use std::ops::ControlFlow;
use todo::todo_factory;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server Serving on http://localhost:8000");
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

/// Allow us to switch back to the command line version.
#[allow(dead_code)]
fn cli_todo_initiation() -> ControlFlow<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Invalid number of args");
        println!("Please enter two args ie, command [create delete edit ], and title ");
        return ControlFlow::Break(());
    }
    let command: &String = &args[1];
    let title: &String = &args[2];
    let default_state: String = "todo".to_string();
    let target_state = args.get(3).unwrap_or(&default_state);
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "todo".to_string();
        }
    }
    let item = todo_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state, target_state.to_string());
    // collecting command line arguments

    ControlFlow::Continue(())
}

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
