use actix_files as fs;
use actix_web::{App, HttpServer, middleware::DefaultHeaders};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().header("Content-Type", "text/html"))
            .service(fs::Files::new("/", "./apk").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}