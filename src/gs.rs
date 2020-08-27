use actix_web::{web, App, HttpServer};
mod file_routing;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/objects").route("", web::get().to(file_routing::objects_route)))
            .service(web::scope("").route("/{filepath:.*}", web::get().to(file_routing::route)))
        })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
