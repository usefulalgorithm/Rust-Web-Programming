use actix_web::web::{get, ServiceConfig};

mod items;
mod content_loader;

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(items::items));
}
