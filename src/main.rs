#[macro_use]
extern crate diesel;

use actix_web::get;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use std::io;

mod database;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("localhost:8080")?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Application Index")
}
