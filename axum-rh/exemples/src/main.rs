mod routes;
use axum::serve;
use axum_rh::router::traits::RouterHelper;
use routes::ExempleApiRouter;
use tokio::net::TcpListener;
use axum_rh::router::middlewares;
use axum::middleware::from_fn;
use axum_rh::router::logger::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let tcp = TcpListener::bind(&"0.0.0.0:3005").await?;
    let session_layer = middlewares::sessions::get_in_memory_session_layer(None).await;
    let app = ExempleApiRouter::load_routers().layer(from_fn(
            middlewares::logging::logging_middleware,
        )).layer(session_layer);
    serve(tcp, app.into_make_service()).await?;
    Ok(())
}
