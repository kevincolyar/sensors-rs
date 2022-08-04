#[cfg(test)]
mod endpoints {
    use actix_web::{http::header::ContentType, test, App};
    use sensors::endpoints::*;
    use sensors::requests::MeasurementRequest;

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
        assert_eq!(body, actix_web::web::Bytes::from_static
                   (b"{\"device_id\":365951380,\"epoch_ms\":1640995229697,\"formatted_time\":\"2022/01/01 00:00:29\",\"name\":\"Temperature\",\"value\":58.48256793121914}")
        );

    }
}
