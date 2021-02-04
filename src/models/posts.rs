use crate::util::establish_connection;
use diesel::prelude::*;
use crate::schema::posts;
use chrono::{DateTime};

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: u64,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    user_id: i32,
    title: String,
    body:  String,
}

impl Post {
    pub fn create(title: String, body: String) -> Post {
        use self::posts::id;
        // let utc: DateTime<Utc>= Utc::now();
        // let naive: NaiveDateTime = utc.naive_local();
        let new_post = NewPost { user_id: 1i32, title: title, body: body };
        let connection = establish_connection();
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(&connection)
            .expect("Error saving new post");
        posts::dsl::posts
            .order(id.desc())
            .first::<Post>(&connection)
            .expect("Error finding posts")
    }
}
