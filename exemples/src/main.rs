mod routes;
use axum::serve;
use axum_router_helper::router::traits::RouterHelper;
use routes::ExempleApiRouter;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp = TcpListener::bind(&"0.0.0.0:3005").await.unwrap();
    let app = ExempleApiRouter::load_routers();
    serve(tcp, app.into_make_service()).await?;
    Ok(())
}
