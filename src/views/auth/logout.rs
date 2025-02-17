#[actix_web::get("/logout")]
pub async fn logout() -> String {
    format!("Logout view")
}