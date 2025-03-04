mod routes;
use axum::serve;
use axum_rh::router::traits::RouterHelper;
use routes::ApiRouter;
use tokio::net::TcpListener;
use axum_rh::router::middlewares::{sessions, logging};
use axum::middleware::from_fn;
use axum_rh::router::logger::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let tcp = TcpListener::bind(&"0.0.0.0:3005").await?;
    let session_layer = sessions::in_memory(None).await;
    let app = ApiRouter::load_routers().layer(from_fn(
            logging,
        )).layer(session_layer);
    serve(tcp, app.into_make_service()).await?;
    Ok(())
}
