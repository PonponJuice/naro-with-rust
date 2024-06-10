use crate::DataBase;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct UserPassword {
    hashed_pass: String,
}

impl DataBase {
    pub async fn save_password(&self, display_id: String, password: String) -> anyhow::Result<()> {
        
        let hashed_pass = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query(r#"
        INSERT INTO user_passwords (display_id, hashed_pass)
        VALUES (?, ?)
        "#)
        .bind(&display_id)
        .bind(&hashed_pass)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }



    pub async fn verify_user_password(&self, display_id: String, password: String) -> anyhow::Result<bool> {
        let hps = sqlx::query_as::<_, UserPassword>(r#"
        SELECT hashed_pass
        FROM user_passwords
        WHERE display_id = ?
        "#)
        .bind(display_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(bcrypt::verify(password, &hps.hashed_pass)?)
    }
}