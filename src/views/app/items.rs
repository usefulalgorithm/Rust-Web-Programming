use actix_web::{http::header::ContentType, HttpResponse};

use super::content_loader::{add_component, read_file};

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("./templates/main.html");

    let javascript_data = read_file("./javascript/main.js");
    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);

    let base_css_data = read_file("./css/base.css");
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);

    let css_data = read_file("./css/main.css");
    html_data = html_data.replace("{{CSS}}", &css_data);

    html_data = add_component(String::from("header"), html_data);

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_data)
}
