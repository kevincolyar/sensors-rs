use actix_web::{delete, get, post, web, Responder, HttpResponse, Error};

use crate::measurement::*;
use crate::requests::{MeasurementRequest};

#[post("/temp")]
pub async fn temp_post(item: web::Json<MeasurementRequest>) -> impl Responder {
    // let m = Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914");
    // println!("item: {:?}", &item);
    let m = Measurement::try_from(&*item.data);

    return web::Json(m.unwrap())
}

#[get("/foo")]
pub async fn foo_get() -> Result<HttpResponse, Error> {
    let m = Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914");

    // let res = HttpResponse::Ok().finish

    // web::Json(m.unwrap())
     Ok(HttpResponse::Ok().json(web::Json(m.unwrap())))
}

#[get("/errors")]
pub async fn errors_get() -> impl Responder {
    format!("GET /errors")
}

#[delete("/errors")]
pub async fn errors_delete() -> impl Responder {
    format!("GET /errors")
}
