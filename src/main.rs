#[macro_use]
extern crate diesel;

use std::sync::Mutex;

use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

mod database;

#[derive(Clone)]
pub struct ApplicationData {
    conn_pool: database::Pool
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    let application_data = ApplicationData {
        conn_pool: database::create_pool()
    };

    let data = web::Data::new(Mutex::new(application_data));

    HttpServer::new(|| 
        App::new()
        .app_data(data.clone())
        .service(index)
        )
        .bind("localhost:8080")?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Application Index")
}
