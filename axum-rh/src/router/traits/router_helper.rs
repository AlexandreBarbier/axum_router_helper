use axum::Router;

pub trait RouterHelper<T> {
    fn load_routers() -> Router<T> {
        panic!("load_routers() not implemented");
    }
    fn load_routers_with_auth() -> Router<T> {
        panic!("load_routers_with_auth() not implemented");
    }
}
