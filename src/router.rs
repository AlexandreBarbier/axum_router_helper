pub mod models {
    pub mod response_body;

    pub use response_body::ResponseBody;
    cfg_if::cfg_if! {
        if #[cfg(feature = "base")] {
            pub mod api_response;
            pub use api_response::ApiResponse;
            pub mod endpoint;
            pub use endpoint::Endpoint;
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "base")] {
        pub use axum::{middleware::from_fn, routing::MethodRouter, Router};
        pub mod middlewares;
        pub mod traits;
        pub mod utils;
        pub use arh_macros;
        #[macro_export]
        macro_rules! load_routers {
        ($($router:ident), +) => {
            {
                let mut router = axum::Router::new();
                $(
                    router = router.merge($router::router());
                )+
                router
            }
        };
        }
    }
}
