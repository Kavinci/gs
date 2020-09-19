//use actix_web::{web, App, HttpServer};
mod server;
mod configuration;

//async 
fn main() { 
    //-> std::io::Result<()> {
    //HttpServer::new(|| {
    //    App::new()
    //        .service(web::scope("/references").route("", web::get().to(file_routing::references_route)))
    //        .service(web::scope("").route("/{filepath:.*}", web::get().to(file_routing::route)))
    //    })
    //    .bind("127.0.0.1:5000")?
    //    .run()
    //    .await
    
    let config = configuration::Configuration::new();
    let pool = server::threading::ThreadPool::new(16);
    server::HTTP::bind(config.port).run(server::handle_connection, pool);
}