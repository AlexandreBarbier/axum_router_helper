use axum::extract::Path;
use axum::http::HeaderMap;
use axum_rh::{
    macros::{get, post, router},
    router::{models::ApiResponse, utils::session_manager::{SessionData, SessionObject, SessionTrait}},
};
use serde_json::json;
pub struct Samples;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct SampleData {
    pub id: i32,
    pub name: String,
    pub session_key: Option<String>,
}

#[router(session_type=SessionData, base_path = "/samples")]
impl Samples {
    #[get("")]
    async fn get_samples() -> ApiResponse<serde_json::Value> {
        let samples: Vec<SampleData> = (0..10)
            .map(|i| SampleData {
                id: i,
                name: format!("Sample {}", i),
                ..Default::default()
            })
            .collect();
        ApiResponse::ok(Some(json!(samples)))
    }

    #[post("/{id}")]
    async fn post_samples(Path(id): Path<i32>) -> ApiResponse<SampleData> {
        let sample = SampleData {
            id: id,
            name: "Sample".to_string(),
            ..Default::default()
        };
        ApiResponse::ok(Some(sample))
    }

    #[post("/{id}/header")]
    async fn post_samples_header(Path(id): Path<i32>) -> ApiResponse<SampleData> {
        let sample = SampleData {
            id: id,
            name: "Sample".to_string(),
            ..Default::default()
        };
        let mut res = ApiResponse::ok(Some(sample));
        res.headers = Some(HeaderMap::from_iter(vec![
            (
                axum::http::header::SET_COOKIE,
                "session=; HttpOnly; Max-Age=0; Path=/".parse().unwrap(),
            ),
        ]));
        res
    }

    #[get("/{id}")]
    async fn get_sample(mut session: SessionObject<SessionData>, Path(id): Path<i32>) -> ApiResponse<SampleData> {
        println!("Session: {:?}", session.data.has_key());
        println!("Session key: {:?}", session.session);
        let sample = SampleData {
            id: id,
            name: "Sample".to_string(),
            session_key: session.data.key(),
        };
        if session.data.key().is_none() {
            session.update_key("123".to_string()).await;
        }
        ApiResponse::ok(Some(sample))
    }
}
