use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::{debug_query, insert_into};
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

use crate::models::*;
use crate::schema::users::dsl::*;

pub fn run() {
  let connection = establish_connection();

  // create
  println!("What would you like your name to be?");
  let mut t = String::new();
  stdin().read_line(&mut t).unwrap();
  let t = &t[..(t.len() - 1)]; // Drop the newline character
  println!(
    "\nOk! {}, what is your hair color? (Press {} when finished)\n",
    t, EOF
  );
  let mut b = String::new();
  stdin().read_to_string(&mut b).unwrap();
  let b = &b[..(b.len() - 1)]; // Drop the newline character

  let user_id = create_user(&connection, t, b).id;
  println!("\nSaved user {} with id {}", t, user_id);
}

fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn create_user<'a>(conn: &PgConnection, t: &'a str, b: &'a str) -> User {
  // // case 1(insert by a struct)
  // let new_user = UserForm {
  //   name: t,
  //   hair_color: Some(b),
  // };

  // println!(
  //   "{}",
  //   debug_query::<Pg, _>(&diesel::insert_into(users).values(&new_user)).to_string()
  // );

  // insert_into(users)
  //   .values(&new_user)
  //   .get_result(conn)
  //   .expect("Error saving new user")

  // // case 2(insert by a json)
  // let json = format!(r#"{{ "name": "{}", "hair_color": "{}" }}"#, t, b);
  // let user_form = serde_json::from_str::<UserForm>(&json).unwrap();

  // println!(
  //   "{}",
  //   debug_query::<Pg, _>(&diesel::insert_into(users).values(&user_form)).to_string()
  // );

  // insert_into(users)
  //   .values(&user_form)
  //   .get_result::<User>(conn)
  //   .expect(&format!("Unable to create user {}", user_form.name))

  // case 3(batch insert)
  println!(
    "{}",
    debug_query::<Pg, _>(
      &diesel::insert_into(users)
        .values(&vec![
          (name.eq("Sean"), Some(hair_color.eq("Black"))),
          (name.eq("Ruby"), None),
        ])
        .as_query()
    )
    .to_string()
  );

  insert_into(users)
    .values(&vec![
      (name.eq("Sean"), Some(hair_color.eq("Black"))),
      (name.eq("Ruby"), None),
    ])
    .get_result(conn)
    .expect("Error saving new user")
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
