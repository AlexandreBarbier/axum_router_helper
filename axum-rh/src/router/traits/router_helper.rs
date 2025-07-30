use axum::Router;

pub struct Routers<T> {
    pub open_router: axum::Router<T>,
    pub protected_router: axum::Router<T>,
}
pub trait RouterHelper<T> {
    fn load_routers() -> Routers<T> {
        panic!("load_routers() not implemented");
    }
    fn load_routers_with_auth() -> Router<T> {
        panic!("load_routers_with_auth() not implemented");
    }
}
