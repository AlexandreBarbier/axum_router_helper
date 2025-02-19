pub mod health;

use axum_router_helper::{
    load_routers,
    macros::{router_config, RouterHelper},
    router::traits::{ApiRouter, RouterHelper},
};

use health::Health;

#[derive(RouterHelper)]
#[router_config((), Health)]
pub struct ExempleApiRouter;
