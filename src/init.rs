extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub fn run() {
  dotenv().ok();

  for (key, value) in env::vars() {
    println!("{}: {}", key, value);
  }
}
