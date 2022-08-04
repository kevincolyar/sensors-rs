use actix_web::{App, HttpServer};

use sensors::endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(temp_post)
            .service(errors_get)
            .service(errors_delete)
            .service(foo_get)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
