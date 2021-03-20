#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use rocket_contrib::json::Json;

mod database;
mod json_serialization;
mod models;
mod schema;
mod todo;

use crate::database::DbConn;
use crate::json_serialization::todo_items::ToDoItems;
use crate::models::item::item::Item;
use crate::todo::todo_factory;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/bye/<name>/<age>")]
fn bye(name: String, age: u8) -> String {
    format!("Goodbye, {} year old named {}!", age, name)
}

#[get("/get/<user_id>")]
fn get_items(user_id: i32, conn: DbConn) -> Json<ToDoItems> {
    let items = schema::to_do::table
        .order(schema::to_do::columns::id.asc())
        .filter(schema::to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&*conn)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let item = todo_factory(&item.status, &item.title).unwrap();
        array_buffer.push(item);
    }
    return Json(ToDoItems::new(array_buffer));
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, bye])
        .mount("/items", routes![get_items])
        .attach(DbConn::fairing())
        .launch();
}
