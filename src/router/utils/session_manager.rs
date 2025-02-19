use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tower_sessions::Session;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionData {
    pub user_id: Option<String>,
    pub created_at: OffsetDateTime,
    pub end_at: OffsetDateTime,
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
pub struct SessionObject {
    pub session: Session,
    data: SessionData,
}

impl SessionObject {
    const OBJECT_KEY: &'static str = "data";

    pub async fn set_user_id(&mut self, user_id: String) {
        self.data.user_id = Some(user_id);
        Self::update_session(&self.session, &self.data).await;
    }

    async fn update_session(session: &Session, session_data: &SessionData) {
        session
            .insert(Self::OBJECT_KEY, session_data)
            .await
            .unwrap();
        session.save().await.unwrap();
    }

    pub fn has_user_id(&self) -> bool {
        self.data.user_id.is_some()
    }

    pub fn get_user_id(&self) -> String {
        self.data.user_id.clone().expect("no user_id found")
    }

    pub async fn clear(&mut self) {
        self.data = SessionData::default();
        self.session.clear().await;
    }
}

impl<S> FromRequestParts<S> for SessionObject
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await?;

        let mut data: SessionData = session
            .get(Self::OBJECT_KEY)
            .await
            .unwrap()
            .unwrap_or_default();
        data.end_at = OffsetDateTime::now_utc();
        Self::update_session(&session, &data).await;
        Ok(Self { session, data })
    }
}
