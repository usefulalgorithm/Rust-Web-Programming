use actix_web::web::{scope, ServiceConfig};

mod create;
pub fn user_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/user").service(create::create));
}
