use actix_web::{http::header::ContentType, HttpResponse};

#[actix_web::get("/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<html>\
            <script>\
                localStorage.removeItem('user-token'); \
                window.location.replace(
                    document.location.origin);\
            </script>\
        </html>",
    )
}
