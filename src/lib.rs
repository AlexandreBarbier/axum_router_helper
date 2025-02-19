#[cfg(feature = "models")]
pub mod router;

#[cfg(feature = "base")]
pub use axum_router_helper_macros as macros;
