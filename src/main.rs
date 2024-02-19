#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Default)]
struct AppState {
    todos: RwLock<Vec<Todo>>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let app_state = AppState::default();
    rocket::build()
        .attach(AdHoc::on_ignite("Todo API", |rocket| {
            Ok(rocket.manage(app_state))
        }))
        .mount("/", routes![index])
}
