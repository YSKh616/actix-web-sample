extern crate diesel;

use crate::controllers::create_user;
use crate::controllers::index;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/users", web::get().to(index::index))
        .route("/users", web::get().to(latest::latest))
        .route("/users", web::post().to(create_user::create));
}
