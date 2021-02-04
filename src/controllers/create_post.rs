extern crate diesel;

use crate::models::posts::Post;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    title: String,
    body: String
}

pub async fn create(item: web::Json<PostData>) -> HttpResponse {
    let post = Post::create(item.title.to_string(), item.body.to_string());
    println!("{:?}", post);
    HttpResponse::Created().body("Inserting")
}
