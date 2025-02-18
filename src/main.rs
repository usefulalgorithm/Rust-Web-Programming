#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use counter::Counter;
use futures::future::{ok, Either};
use views::views_factory;
mod config;
mod counter;
mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    let site_counter = Counter { count: 0 };
    site_counter.save().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, svc| {
                let passed = req.path().contains(&format!("/{}/", ALLOWED_VERSION));
                let mut site_counter = Counter::load().unwrap();
                site_counter.count += 1;
                println!("{:?}", site_counter);
                site_counter.save().unwrap();
                let end_result = match passed {
                    true => Either::Left(svc.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented()
                            .body(format!("Only {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views_factory)
            .wrap(Cors::permissive())
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
