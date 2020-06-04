// 1. schema will be maintained by migration cli.
// 2. models includes table models and operation models.
// 3. Queryable: able to be used in get_result/get_results.
// 4. Identifiable: having PK definitions.
// 5. AsChangeset: aboe to be used in update() with the struct.
// 6. Insertable: having columns for creating items by insert_into().
use std::time::SystemTime;

use crate::schema::{posts, users};

#[derive(Queryable, Identifiable, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserForm<'a> {
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
}

