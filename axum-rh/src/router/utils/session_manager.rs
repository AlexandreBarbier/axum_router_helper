use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions::Session;

pub trait SessionTrait: Send + Sync + Serialize + for<'de> Deserialize<'de> + Default {
    fn key(&self) -> Option<String>;
    fn set_key(&mut self, key: String);
    fn has_key(&self) -> bool {
        self.key().is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionData {
    pub user_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub end_at: OffsetDateTime,
}

impl SessionTrait for SessionData {
    fn key(&self) -> Option<String> {
        self.user_id.clone()
    }

    fn set_key(&mut self, key: String) {
        self.user_id = Some(key);
    }
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            user_id: None,
            created_at: OffsetDateTime::now_utc(),
            end_at: OffsetDateTime::now_utc(),
        }
    }
}
pub struct SessionObject<T>
where
    T: SessionTrait,
{
    pub session: Session,
    pub data: T,
}

impl<T> SessionObject<T>
where
    T: SessionTrait,
{
    const DATA_KEY: &'static str = "data";

    pub async fn update(&mut self) {
        Self::update_session(&self.session, &self.data).await;
    }

    pub async fn update_key(&mut self, key: String) {
        self.data.set_key(key);
        self.update().await;
    }

    async fn update_session(session: &Session, session_data: &T) {
        match session.insert(Self::DATA_KEY, session_data).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error updating session: {:?}", e);
            }
        };
        match session.save().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error saving session: {:?}", e);
            }
        };
    }

    pub async fn clear(&mut self) {
        self.data = T::default();
        self.session.clear().await;
    }
}

impl<S, T> FromRequestParts<S> for SessionObject<T>
where
    S: Send + Sync,
    T: SessionTrait,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await?;

        let data: T = session
            .get(Self::DATA_KEY)
            .await
            .expect("session data not found")
            .unwrap_or_default();

        Self::update_session(&session, &data).await;
        Ok(Self { session, data })
    }
}
