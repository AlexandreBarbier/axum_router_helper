use log::info;
use std::env;
use time::Duration;
use tokio::task::JoinHandle;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

pub async fn redis(
    expiry: Option<i64>,
) -> (
    SessionManagerLayer<RedisStore<Pool>>,
    JoinHandle<Result<(), Error>>,
) {
    if env::var("REDIS_URL").is_err() {
        info!("REDIS_URL must be set");
    }
    let expiry = expiry.unwrap_or(3600);
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://localhost:6379".to_string());
    let config = Config::from_url(&redis_url).expect("cannot create config from url");
    let pool = Pool::new(config, None, None, None, 1).expect("cannot create pool");
    let redis_conn = pool.connect();
    let _ = pool.wait_for_connect().await;
    let session_store = RedisStore::new(pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(expiry)));
    (session_layer, redis_conn)
}

pub async fn in_memory(expiry: Option<i64>) -> SessionManagerLayer<MemoryStore> {
    let expiry = expiry.unwrap_or(3600);
    let session_store = MemoryStore::default();
    SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(expiry)))
}
