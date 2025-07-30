use axum::Router;

use crate::router::{models::endpoint::Endpoint, utils::session_manager::SessionTrait};

pub trait ArhRouter<T, A>
where
    T: Send + Sync + 'static + Clone,
    A: SessionTrait + Send + Sync + 'static + Clone,
{
    fn endpoints() -> Vec<Endpoint<T>> {
        panic!("endpoints() not implemented");
    }

    fn router() -> axum::Router<T> {
        Self::endpoints()
            .into_iter()
            .filter(|x| !x.authenticated)
            .fold(Router::new(), |router, endpoint| {
                let path = endpoint.path;
                router.route(path, endpoint.handler)
            })
    }

    fn auth_router() -> axum::Router<T> {
        Self::endpoints()
            .into_iter()
            .filter(|x| x.authenticated)
            .fold(Router::new(), |router, endpoint| {
                let path = endpoint.path;
                router.route(path, endpoint.handler)
            })
    }
}
