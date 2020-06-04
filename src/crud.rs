use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

use crate::models::*;
use crate::schema::posts::dsl::*;

pub fn run() {
  let connection = establish_connection();

  // create
  println!("What would you like your title to be?");
  let mut t = String::new();
  stdin().read_line(&mut t).unwrap();
  let t = &t[..(t.len() - 1)]; // Drop the newline character
  println!("\nOk! Let's write {} (Press {} when finished)\n", t, EOF);
  let mut b = String::new();
  stdin().read_to_string(&mut b).unwrap();

  let post_id = create_post(&connection, t, &b).id;
  println!("\nSaved draft {} with id {}", t, post_id);

  // update
  let post = diesel::update(posts.find(post_id))
    .set(published.eq(true))
    .get_result::<Post>(&connection)
    .expect(&format!("Unable to find post {}", post_id));
  println!("Published post {}", post.title);

  // delete
  let pattern = format!("%{}%", post.title);
  let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(&connection)
    .expect("Error deleting posts");
  println!("Deleted {} posts", num_deleted);

  // query
  let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
    println!("{}", post.title);
    println!("----------\n");
    println!("{}", post.body);
  }
}

fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &PgConnection, t: &'a str, b: &'a str) -> Post {
  let new_post = NewPost { title: t, body: b };

  diesel::insert_into(posts)
    .values(&new_post)
    .get_result(conn)
    .expect("Error saving new post")
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
