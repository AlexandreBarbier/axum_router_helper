use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

mod impls;
mod models;
use models::RouterAttributes;

/// Attribute macro to define a router.
///
/// # Arguments
///
/// * `state` - Application state.
/// * `base_path` - the base path of these routes (must start with "/").
///
/// # Example
///
/// ```rust
/// #[router(state=AppState, base_path="/path")]
/// impl MyRouter {
///     // implementation
/// }
/// ```
/// State is optional
/// ```rust
/// #[router("/path")]
/// impl MyRouter {
///     // implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn router(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_item = parse_macro_input!(item as ItemImpl);
    let router_attr = parse_macro_input!(attr as RouterAttributes);
    impls::router(router_attr, parsed_item)
}

/// Derive macro to generate helper methods for a router.
///
/// # Arguments
///
/// * `router_config` - proc macro argument that start with the application's state and followd by all the routers.
///
/// # Example
///
/// ```rust
/// #[derive(RouterHelper)]
/// #[router_config(AppState, MyRouter, AnotherRouter, ...)]
/// struct MyRouter {
///     // fields
/// }
/// ```
#[proc_macro_derive(RouterHelper, attributes(router))]
pub fn router_helper_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    impls::router_helper_derive(input)
}

/// Attribute macro to define a POST route.
///
/// # Arguments
///
/// * `path` - the route path.
/// * `auth` - optional boolean to tell if the route need to be authenticated.
///
/// # Example
///
/// ```rust
/// #[post("/path")]
/// fn my_post_route() {
///     // implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
/// Attribute macro to define a PUT route.
/// # Arguments
/// * `path` - the route path.
/// * `auth` - optional boolean to tell if the route need to be authenticated.
/// # Example
/// ```rust
/// #[put("/path", auth=true)]
/// fn my_put_route() {
///    // implementation
/// }
/// ```
///
#[proc_macro_attribute]
pub fn put(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
/// Attribute macro to define a GET route.
/// # Arguments
/// * `path` - the route path.
/// * `auth` - optional boolean to tell if the route need to be authenticated.
/// # Example
/// ```rust
/// #[get("/path")]
/// fn my_get_route() {
///   // implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Attribute macro to define a DELETE route.
/// # Arguments
/// * `path` - the route path.
/// * `auth` - optional boolean to tell if the route need to be authenticated.
/// # Example
/// ```rust
/// #[delete("/path")]
/// fn my_delete_route() {
///  // implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
/// Attribute macro to define a router configuration.
/// # Arguments
/// * `state` - Application state.
/// * `routers` - all the routers.
/// # Example
/// ```rust
/// #[router_config(AppState, MyRouter, AnotherRouter, ...)]
/// struct MyRouter {
///    // fields
/// }
/// ```
#[proc_macro_attribute]
pub fn router_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
