pub mod health;
pub mod samples;
use axum_rh::{
    load_auth_routers, load_routers,
    macros::{router_config, RouterHelper},
};

use health::Health;
use samples::Samples;
// router_config((), Health) takes one required argument which is the state of the router.
// All other arguments are optional and are the routers defined in their respective modules.
// Have a look at the RouterConfiguration struct to understand the router_config macro.

#[derive(RouterHelper)]
#[router_config((), Health, Samples)]
pub(crate) struct ApiRouter;
