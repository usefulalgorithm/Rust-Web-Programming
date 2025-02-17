use actix_web::web::{scope, ServiceConfig};
mod login;
mod logout;

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .service(login::login)
            .service(logout::logout),
    );
}
