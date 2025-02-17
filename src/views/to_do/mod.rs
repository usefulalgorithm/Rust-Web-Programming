use actix_web::web::{scope, ServiceConfig};
mod create;
mod edit;
mod get;
mod delete;

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .service(create::create)
            .service(get::get)
            .service(edit::edit)
            .service(delete::delete),
    );
}
