use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::users::User;

pub async fn latest() -> impl Responder {
    let result = User::latest();
    let mut res = format!("Displaying id: {} user\n\n", result.id);
    let s = format!("id: {}, name: {}\n", result.id, result.name);
    res.push_str(&s);

    HttpResponse::Ok().body(res)
}
