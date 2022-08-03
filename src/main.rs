use actix_web::{delete, get, post, web, App, HttpServer, HttpResponse, Error, Responder};

pub mod measurement;
pub mod commands;
pub mod state;

use crate::measurement::*;

#[post("/temp")]
async fn temp_post() -> impl Responder {
    let m = Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914");

    return web::Json(m.unwrap())
}

#[get("/foo")]
async fn foo_get() -> Result<HttpResponse, Error> {
    let m = Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914");

    // let res = HttpResponse::Ok().finish

    // web::Json(m.unwrap())
     Ok(HttpResponse::Ok().json(web::Json(m.unwrap())))
}

#[get("/errors")]
async fn errors_get() -> impl Responder {
    format!("GET /errors")
}

#[delete("/errors")]
async fn errors_delete() -> impl Responder {
    format!("GET /errors")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(temp_post)
            .service(errors_get)
            .service(errors_delete)
            .service(foo_get)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_foo_ok(){
        let app = test::init_service(
            App::new().service(foo_get)
        ).await;

        let req = test::TestRequest::default()
            .uri("/foo")
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
