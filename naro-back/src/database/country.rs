use crate::DataBase;

// IDのせいで#[sqlx(rename_all = "PascalCase")] が使えないので、手動でrenameを書く(嫌い)
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct City {
    #[sqlx(rename = "ID")]
    pub id: i32,
    #[sqlx(rename = "Name")]
    pub name: String,
    #[sqlx(rename = "CountryCode")]
    pub country_code: String,
    #[sqlx(rename = "District")]
    pub district: String,
    #[sqlx(rename = "Population")]
    pub population: i32,
}

impl DataBase {
    pub async fn get_city_by_id(&self, city_name: String) -> anyhow::Result<Option<City>> {
        println!("city_name: `{}`", city_name);
        let city = sqlx::query_as::<_, City>(
            r#"
        SELECT *
        FROM city
        WHERE Name = ?
        "#,
        )
        .bind(city_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(city)
    }
}
