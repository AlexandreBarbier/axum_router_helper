use axum::{middleware::from_fn, Router};

use crate::router::{middlewares::auth::auth_middleware, models::endpoint::Endpoint};

pub trait ApiRouter<T>
where
    T: Send + Sync + 'static + Clone,
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
                    true => router.route(path, endpoint.handler.layer(from_fn(auth_middleware))),
                    false => router.route(path, endpoint.handler),
                }
            })
    }
}
