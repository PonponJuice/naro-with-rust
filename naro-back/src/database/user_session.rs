use anyhow::Context;
use async_session::{Session, SessionStore};

use crate::DataBase;

impl DataBase {
    pub async fn create_session(&self, display_id: String) -> anyhow::Result<String> {
        let mut session = Session::new();
        session
            .insert("user", display_id)
            .with_context(|| "Failed to insert user into session")?;

        let session_id = self
            .session_store
            .store_session(session)
            .await
            .with_context(|| "Failed to store session")?
            .with_context(|| "Failed to create session")?;

        Ok(session_id)
    }

    pub async fn get_display_id_by_session_id(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<String>> {
        dbg!(session_id);
        let session = self
            .session_store
            .load_session(session_id.to_string())
            .await
            .with_context(|| "Failed to load session")?;

        Ok(session.and_then(|s| s.get("user")))
    }

    pub async fn delete_session(&self, session_id: String) -> anyhow::Result<()> {
        let session = self
            .session_store
            .load_session(session_id)
            .await
            .with_context(|| "Failed to load session")?
            .with_context(|| "Session not found")?;

        self.session_store
            .destroy_session(session)
            .await
            .with_context(|| "Failed to delete session")?;

        Ok(())
    }
}
