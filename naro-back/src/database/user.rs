use crate::DataBase;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub display_id: String,
    pub username: String,
}

impl DataBase {
    pub async fn get_user_by_display_id(&self, display_id: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
        SELECT *
        FROM users
        WHERE display_id = ?
        "#,
        )
        .bind(display_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> anyhow::Result<()> {
        sqlx::query(
            r#"
        INSERT INTO users (id, username, display_id)
        VALUES (?, ?, ?)
        "#,
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.display_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
