#[actix_web::get("/login")]
pub async fn login() -> String {
    format!("Login view")
}