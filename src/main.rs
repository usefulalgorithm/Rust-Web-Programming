#[macro_use] extern crate diesel;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};
use views::views_factory;
mod json_serialization;
mod models;
mod config;
mod database;
mod jwt;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, svc| {
                println!("{:?}", req);
                let fut = svc.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views_factory)
            .wrap(Cors::permissive())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
