use crate::router::utils::{self, session_manager::SessionObject};
use axum::{body::Body, http};

pub async fn auth_middleware(
    mut session: SessionObject,
    req: axum::http::Request<Body>,
    next: axum::middleware::Next,
) -> axum::http::Response<Body> {
    let token = match req
        .headers()
        .get(http::header::AUTHORIZATION)
        .map(|h| h.to_str().unwrap().to_string())
    {
        Some(token) => token,
        None => {
            return axum::http::Response::builder()
                .status(401)
                .body(Body::empty())
                .unwrap();
        }
    };
    if session.has_user_id() {
        return next.run(req).await;
    }

    let token = token.split(' ').collect::<Vec<&str>>()[1];
    match utils::auth::decode_jwt(token.to_string()) {
        Ok(decoded_token) => {
            let user_id = decoded_token.claims.clone().user_id;
            session.set_user_id(user_id.clone()).await;
        }
        Err(e) => {
            log::error!("Error decoding token: {:?}", e);
            return axum::http::Response::builder()
                .status(401)
                .body(Body::empty())
                .unwrap();
        }
    }

    next.run(req).await
}
