mod routes;
use axum::middleware::from_fn;
use axum::serve;
use axum_rh::router::logger::init_logger;
use axum_rh::router::middlewares::{logging, sessions};
use axum_rh::router::traits::router_helper::Routers;
use axum_rh::router::traits::RouterHelper;
use log::info;
use routes::ApiRouter;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    log::info!("Starting the server...");
    let tcp = TcpListener::bind(&"0.0.0.0:3005").await?;
    let session_layer = sessions::in_memory(None).await;

    let Routers {
        open_router,
        protected_router,
    } = ApiRouter::load_routers();
    let app = open_router
        .merge(protected_router)
        .layer(from_fn(logging))
        .layer(session_layer);

    info!("Server started on http://localhost:3005");
    serve(tcp, app.into_make_service()).await?;
    Ok(())
}
