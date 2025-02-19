use axum::extract::Path;
use axum_router_helper::{
    macros::{get, post, router},
    router::{
        models::{ApiResponse, Endpoint},
        traits::ApiRouter,
    },
};
use serde_json::json;
pub struct Samples;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct SampleData {
    pub id: i32,
    pub name: String,
}

#[router(base_path = "/samples")]
impl Samples {
    #[get("")]
    async fn get_samples() -> ApiResponse<serde_json::Value> {
        let samples: Vec<SampleData> = (0..10)
            .map(|i| SampleData {
                id: i,
                name: format!("Sample {}", i),
            })
            .collect();
        ApiResponse::ok(Some(json!(samples)))
    }

    #[post("/{id}")]
    async fn post_samples(Path(id): Path<i32>) -> ApiResponse<SampleData> {
        let sample = SampleData {
            id: id,
            name: "Sample".to_string(),
        };
        ApiResponse::ok(Some(sample))
    }
}
