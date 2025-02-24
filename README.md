# axum_rh

Axum router helper helps you build an API simply by using macros to define the router and all endpoints.

## Install

cargo add axum-rh

## Middlewares

All middlewares are optional and can be overriden by your own

### Logging

A simple logger middleware that can also send logs into betterstack (with the async feature)
format request logs ```

### Sessions

Use Redis storage to

### Auth

A default JWT authentication middleware

## How to:

### Declare router

```rust
// base RouterHelper derive macros
// the router_config define the state type (here is an empty state), Health and
// Sample are routers definitions
#[derive(RouterHelper)]
#[router_config((), Health, Samples)]
pub struct ExempleApiRouter;
```

### Use states

```rust
pub struct ApiState {
    pub counter: i32
}

#[derive(RouterHelper)]
#[router_config(ApiState, Health, Samples)]
pub struct ExempleApiRouter;
```

### Declare Endpoints

To declare an endpoint two macros are needed. The first one will be applied on the implementation of the module definition.

```rust
pub struct Samples;

#[router(base_path = "/samples")]
impl Samples{

}
```

As you can see here it can take an optional argument `base_path` which will default to `/` if not set.
The second one will be applied on each function definition under the struct impl

```rust
pub struct Samples;

#[router(base_path = "/samples")]
impl Samples{
    #[get("")]
    async fn get_samples() -> ApiResponse<serde_json::Value> {
        ...
    }
}
```

#### get

Define a get endpoint

#### post

Define a post endpoint

#### put

Define a put endpoint

#### delete

Define a delete endpoint

#### Return values

The APIResponse struct define a default response type for our enpoints.
You can use it or define your own response type as long as it implements into_response

```rust
#[router(base_path = "/samples")]
impl Samples{
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
