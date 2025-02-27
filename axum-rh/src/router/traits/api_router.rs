use axum::{middleware::from_fn, Router};

use crate::router::{
    middlewares::auth::auth_middleware, models::endpoint::Endpoint,
    utils::session_manager::SessionTrait,
};

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
            .fold(Router::new(), |router, endpoint| {
                let path = endpoint.path;
                match endpoint.authenticated {
                    true => {
                        router.route(path, endpoint.handler.layer(from_fn(auth_middleware::<A>)))
                    }
                    false => router.route(path, endpoint.handler),
                }
            })
    }
}
