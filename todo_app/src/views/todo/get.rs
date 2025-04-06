use actix_web::{web, Responder};
use serde_json::{Map, Value};

use crate::state::read_file;

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file(&String::from("./state.json"));
    return web::Json(state);
}
