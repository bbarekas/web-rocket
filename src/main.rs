#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
use rocket_contrib::json::Json;
use diesel::prelude::*;

mod todo;
mod schema;
mod database;
mod models;
mod json_serialization;

use crate::database::DbConn;
use crate::models::item::item::Item;
use crate::todo::todo_factory;
use crate::json_serialization::todo_items::ToDoItems;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age, name)
}


fn main() {
    rocket::ignite().mount("/", routes![hello, bye]).launch();
}
