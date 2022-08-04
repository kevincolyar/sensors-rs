use actix_web::{delete, get, post, web, App, HttpServer, HttpResponse, Error, Responder};
use serde::{Deserialize, Serialize};

pub mod measurement;
pub mod commands;
pub mod state;

use crate::measurement::*;

#[derive(Debug, Serialize, Deserialize)]
struct MeasurementRequest {
    data: String
}

#[post("/temp")]
async fn temp_post(item: web::Json<MeasurementRequest>) -> impl Responder {
    // let m = Measurement::try_from("365951380:1640995229697:'Temperature':58.48256793121914");
    // println!("item: {:?}", &item);
    let m = Measurement::try_from(&*item.data);

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
mod main {
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

    #[actix_web::test]
    async fn test_temp_ok(){
        let app = test::init_service(
            App::new().service(temp_post)
        ).await;

        let req = test::TestRequest::post()
            .uri("/temp")
            .set_json(
                &MeasurementRequest {
                    data: String::from("365951380:1640995229697:'Temperature':58.48256793121914")
                }
            )
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // println!("{:?}", resp);
        let body = test::read_body(resp).await;
        // println!("{:?}", s);
        assert_eq!(body, web::Bytes::from_static
                   (b"{\"device_id\":365951380,\"epoch_ms\":1640995229697,\"formatted_time\":\"2022/01/01 00:00:29\",\"name\":\"Temperature\",\"value\":58.48256793121914}")
        );

    }
}
