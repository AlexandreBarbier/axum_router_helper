# axum_rh

Axum router helper helps you build an API simply by using macros to define the router and all endpoints.

## Install

To add `axum-rh` to your project, run:

```sh
cargo add axum-rh
```

## Middlewares

All middlewares are optional and can be overridden by your own implementations.

### Logging

A simple logger middleware that can also send logs to Betterstack (with the async feature). It formats request logs.

```rust
use axum_rh::router::logger::init_logger;
use axum_rh::router::middlewares;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let tcp = TcpListener::bind(&"0.0.0.0:3005").await.unwrap();
    let app = ExempleApiRouter::load_routers().layer(from_fn(
            middlewares::logging::logging_middleware,
        ));
    serve(tcp, app.into_make_service()).await?;
    Ok(())
}
```

### Sessions

A default implementation of session management using Redis storage. This middleware will be updated to be more user-friendly in the future.

### Auth

A default JWT authentication middleware. The JWT token requires a `user_id` field. This middleware will also be updated to be more user-friendly in the future.

## How to:

### Declare Router

To declare a router, use the `RouterHelper` derive macro and the `router_config` attribute to define the state type and router definitions.

```rust
#[derive(RouterHelper)]
#[router_config((), Health, Samples)]
pub struct ExempleApiRouter;
```

### Use States

To use states in your router, define a state struct and include it in the `router_config` attribute.

```rust
pub struct ApiState {
    pub counter: i32
}

#[derive(RouterHelper)]
#[router_config(ApiState, Health, Samples)]
pub struct ExempleApiRouter;
```

### Declare Endpoints

To declare an endpoint, use two macros. The first macro is applied to the implementation of the module definition.

```rust
pub struct Samples;

#[router(base_path = "/samples")]
impl Samples {

}
```

The `base_path` argument is optional and defaults to `/` if not set. The second macro is applied to each function definition under the struct implementation.

```rust
pub struct Samples;

#[router(base_path = "/samples")]
impl Samples {
    #[get("")]
    async fn get_samples() -> ApiResponse<serde_json::Value> {
        ...
    }
}
```

#### HTTP Methods

Define endpoints using the following macros:

- **GET**: Define a GET endpoint.

  ```rust
  #[get("")]
  ```

- **POST**: Define a POST endpoint.

  ```rust
  #[post("")]
  ```

- **PUT**: Define a PUT endpoint.

  ```rust
  #[put("")]
  ```

- **DELETE**: Define a DELETE endpoint.

  ```rust
  #[delete("")]
  ```

#### Return Values

The `ApiResponse` struct defines a default response type for your endpoints. You can use it or define your own response type as long as it implements `IntoResponse`.

```rust
pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub body: ResponseBody<T>,
    pub error: bool,
    pub headers: Option<Vec<(HeaderName, String)>>,
}

pub struct ResponseBody<T> {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<T>,
}
```

Example:

```rust
#[router(base_path = "/samples")]
impl Samples {
    #[get("")]
    async fn get_samples() -> ApiResponse<serde_json::Value> {
        let samples: Vec<SampleData> = (0..10)
            .map(|i| SampleData {
                id: i,
                name: format!("Sample {}", i),
            })
            .collect();
        ApiResponse::ok(Some(json!(samples)))
    }

    #[post("/{id}")]
    async fn post_samples(Path(id): Path<i32>) -> ApiResponse<SampleData> {
        let sample = SampleData {
            id: id,
            name: "Sample".to_string(),
        };
        ApiResponse::ok(Some(sample))
    }
}
```
