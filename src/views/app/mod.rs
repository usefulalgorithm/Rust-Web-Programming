use actix_web::web::{get, ServiceConfig};

mod content_loader;
mod items;

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(items::items));
}
