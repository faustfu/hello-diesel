#![allow(dead_code)]

mod models;
mod schema;
mod crud;
mod update;
mod insert;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() {
    // crud::run();
    // update::run();
    insert::run();
}
