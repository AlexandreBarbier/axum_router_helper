use crate::router::utils::{self, session_manager::SessionObject};
use axum::{body::Body, http};

pub async fn auth_middleware<T>(
    mut session: SessionObject<T>,
    req: axum::http::Request<Body>,
    next: axum::middleware::Next,
) -> axum::http::Response<Body>
where
    T: utils::session_manager::SessionTrait,
{
    let token = match req
        .headers()
        .get(http::header::AUTHORIZATION)
        .map(|h| h.to_str().unwrap_or_default().to_string())
    {
        Some(token) => token,
        None => {
            return axum::http::Response::builder()
                .status(401)
                .body(Body::empty())
                .expect("failed to build response");
        }
    };
    if session.data.key().is_some() && utils::auth::decode_jwt(token.to_string()).is_ok() {
        return next.run(req).await;
    }

    let token = token.split(' ').collect::<Vec<&str>>();
    if token.len() != 2 || token[0] != "Bearer" {
        return axum::http::Response::builder()
            .status(401)
            .body(Body::empty())
            .expect("failed to build response");
    }
    let token = token[1];

    match utils::auth::decode_jwt(token.to_string()) {
        Ok(decoded_token) => {
            let user_id = decoded_token.claims.clone().user_id;
            session.update_key(user_id.clone()).await;
        }
        Err(e) => {
            log::error!("Error decoding token: {e:?}");
            return axum::http::Response::builder()
                .status(401)
                .body(Body::empty())
                .expect("failed to build response");
        }
    }

    next.run(req).await
}
