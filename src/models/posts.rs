use diesel::prelude::*;
use crate::util::establish_connection;

#[derive(Debug, Queryable)]
pub struct Posts {
    pub id: u64,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: Datetime,
    pub updated_at: Datetime
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    title: &'a str,
    body:  &'a str
}
