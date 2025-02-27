use axum::routing::MethodRouter;

#[derive(Clone, Debug)]
pub struct Endpoint<T> {
    pub path: &'static str,
    pub authenticated: bool,
    pub handler: MethodRouter<T>,
}

impl<T> Endpoint<T> {
    pub fn new(path: &'static str, authenticated: bool, handler: MethodRouter<T>) -> Self {
        Self {
            path,
            authenticated,
            handler,
        }
    }
}
