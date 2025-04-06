use crate::process::process_input;
use crate::state::read_file;
use crate::todo;
use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;
pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file(&String::from("./state.json")); // 1
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone(); // 2
    let item = todo::todo_factory(&String::from("todo"), &title).expect("create "); // 3
    process_input(item, "create".to_string(), &state, "Todo".to_string()); // 4
    return format!("{} created", title_reference); // 5
}
