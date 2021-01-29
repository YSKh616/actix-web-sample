extern crate diesel;

use crate::models::users::User;
use crate::controllers::create_user;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::post().to(create_user::create));
}
