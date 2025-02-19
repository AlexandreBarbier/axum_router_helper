use crate::router::utils::session_manager::SessionObject;
use axum::body::Body;
use log::info;

pub async fn logging_with_session_middleware(
    session: SessionObject,
    req: axum::http::Request<Body>,
    next: axum::middleware::Next,
) -> axum::http::Response<Body> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start_time = std::time::Instant::now();
    let res = next.run(req).await;
    let duration = start_time.elapsed();
    let status = res.status().as_u16();
    match session.has_user_id() {
        true => {
            info!(status=status, user=session.get_user_id().as_str(), method=method.to_string().as_str();
                "{} in {}ms",
                path,
                duration.as_millis()
            )
        }
        _ => info!(status=status, method=method.to_string().as_str();
            "{} in {}ms",
            path,
            duration.as_millis()
        ),
    }

    res
}

pub async fn logging_middleware(
    req: axum::http::Request<Body>,
    next: axum::middleware::Next,
) -> axum::http::Response<Body> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start_time = std::time::Instant::now();
    let res = next.run(req).await;
    let duration = start_time.elapsed();
    let status = res.status().as_u16();

    info!(status=status, method=method.to_string().as_str();
        "{} in {}ms",
        path,
        duration.as_millis()
    );

    res
}
