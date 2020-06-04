use diesel::debug_query;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::*;
use crate::schema::posts::dsl::*;

pub fn run() {
  let connection = establish_connection();

  let post_id: i32 = 2;

  // update
  let post = diesel::update(posts.find(post_id))
    .set(body.eq("How are you?"))
    .get_result::<Post>(&connection)
    .expect(&format!("Unable to find post {}", post_id));
  println!("Changed post {}", post.title);

  // // update in a table
  // println!("{}", debug_query(&diesel::update(posts).set(published.eq(false))).to_string());

  //// update a row
  // println!(
  //   "{}",
  //   debug_query::<Pg, _>(&diesel::update(posts.find(post_id)).set(body.eq("How are you?")))
  //     .to_string()
  // );

  // update by other columns
  println!(
    "{}",
    debug_query::<Pg, _>(&diesel::update(&post).set(body.eq(title))).to_string()
  );

  // // update multiple columns
  // println!(
  //   "{}",
  //   debug_query::<Pg, _>(
  //     &diesel::update(posts.find(post_id)).set((body.eq("How are you?"), published.eq(false))),
  //   )
  //   .to_string()
  // );

  // // update by a struct
  // println!(
  //   "{}",
  //   debug_query::<Pg, _>(
  //     &diesel::update(posts.find(post_id)).set(&post),
  //   )
  //   .to_string()
  // );
}

fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
