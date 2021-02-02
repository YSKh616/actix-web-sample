use crate::util::establish_connection;
use diesel::prelude::*;
use crate::schema::posts;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: u64,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    // pub created_at: Datetime,
    // pub updated_at: Datetime
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    title: &'a str,
    body:  &'a str
}

impl Post {
    pub fn create(title: &str, body: &str) -> Post {
        // use self::posts::id;
        let new_post = NewPost { title: title, body: body };
        let connection = establish_connection();
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(&connection)
            .expect("Error saving new post");
    }
}
