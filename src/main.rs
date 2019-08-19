mod controllers;
mod routes;
mod utils;
mod models;
mod middlewares;
mod schema;
#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpResponse, HttpServer};
use listenfd::ListenFd;
#[macro_use]
extern crate serde;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(routes::auth::config)
            // .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
            .default_service(web::route().to(|| HttpResponse::Ok().body("You are lost!")))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}
