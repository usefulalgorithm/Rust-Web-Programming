use actix_web::web::{scope, ServiceConfig};
mod create;
mod delete;
mod edit;
mod get;

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .service(create::create)
            .service(get::get)
            .service(edit::edit)
            .service(delete::delete),
    );
}
