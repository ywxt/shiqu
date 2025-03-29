use sqlx::{Row, SqlitePool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conversation {
    pub id: i32,
    pub user_id: i32,
    pub character_id: i32,
    pub created_at: time::OffsetDateTime,
}

pub struct ConversationContext(SqlitePool);

impl ConversationContext {
    pub const TABLE_NAME: &'static str = "conversations";
    pub const ID: &'static str = "id";
    pub const USER_ID: &'static str = "user_id";
    pub const CHARACTER_ID: &'static str = "character_id";
    pub const CREATED_AT: &'static str = "created_at";

    pub fn new(pool: SqlitePool) -> ConversationContext {
        ConversationContext(pool)
    }

    pub async fn find_by_id(&mut self, id: i64) -> Result<Option<Conversation>, sqlx::Error> {
        let mut conn = self.0.acquire().await?;
        let row = sqlx::query("SELECT ({}, {}, {}, {}) FROM {} WHERE {} = ?")
            .bind(Self::ID)
            .bind(Self::USER_ID)
            .bind(Self::CHARACTER_ID)
            .bind(Self::CREATED_AT)
            .bind(Self::TABLE_NAME)
            .bind(Self::ID)
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(row.map(|r| Conversation {
            id: r.get(Self::ID),
            user_id: r.get(Self::USER_ID),
            character_id: r.get(Self::CHARACTER_ID),
            created_at: r.get(Self::CREATED_AT),
        }))
    }
}
