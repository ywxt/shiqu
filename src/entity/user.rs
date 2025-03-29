use sqlx::{Row, SqlitePool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub telegram_id: u64,
    pub name: String,
}

pub struct UserContext(SqlitePool);

impl UserContext {
    pub const TABLE_NAME: &'static str = "users";
    pub const ID: &'static str = "id";
    pub const TELEGRAM_ID: &'static str = "telegram_id";
    pub const NAME: &'static str = "name";

    pub fn new(pool: SqlitePool) -> UserContext {
        UserContext(pool)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let mut conn = self.0.acquire().await?;
        let row = sqlx::query("SELECT ({}, {}, {}) FROM {} WHERE {} = ?")
            .bind(Self::ID)
            .bind(Self::TELEGRAM_ID)
            .bind(Self::NAME)
            .bind(Self::TABLE_NAME)
            .bind(Self::ID)
            .bind(id)
            .fetch_optional(&mut *conn).await?;

        Ok(row.map(|r| User {
            id: r.get(Self::ID),
            telegram_id: r.get(Self::TELEGRAM_ID),
            name: r.get(Self::NAME),
        }))
    }

    pub async fn find_by_telegram_id(&mut self, telegram_id: i64) -> Result<Option<User>, sqlx::Error> {
        let mut conn = self.0.acquire().await?;
        let row = sqlx::query("SELECT ({}, {}, {}) FROM {} WHERE {} = ?")
            .bind(Self::ID)
            .bind(Self::TELEGRAM_ID)
            .bind(Self::NAME)
            .bind(Self::TABLE_NAME)
            .bind(Self::TELEGRAM_ID)
            .bind(telegram_id)
            .fetch_optional(&mut *conn).await?;

        Ok(row.map(|r| User {
            id: r.get(Self::ID),
            telegram_id: r.get(Self::TELEGRAM_ID),
            name: r.get(Self::NAME),
        }))
    }
}
