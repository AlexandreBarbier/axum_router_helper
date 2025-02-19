use axum::Router;

pub trait RouterHelper<T> {
    fn load_routers() -> Router<T> {
        panic!("load_routers() not implemented");
    }
}
