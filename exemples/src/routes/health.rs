use axum_rh::{
    macros::{get, router},
    router::models::ApiResponse,
};
use serde_json::json;
pub struct Health;

#[router()]
impl Health {
    #[get("/health")]
    async fn health() -> ApiResponse<serde_json::Value> {
        ApiResponse::ok(Some(json!({"status": "UP"})))
    }
}
