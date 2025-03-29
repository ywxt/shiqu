use sqlx::SqlitePool;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub role: Role,
    pub content: String,
    pub created_at: time::UtcDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    System = 0,
    Assistant = 1,
    User = 2,
}

pub struct MessageContext(SqlitePool);

impl MessageContext {
    pub const TABLE_NAME: &'static str = "messages";
    pub const ID: &'static str = "id";
    pub const CONVERSATION_ID: &'static str = "conversation_id";
    pub const ROLE: &'static str = "role";
    pub const CONTENT: &'static str = "content";
    pub const CREATED_AT: &'static str = "created_at";

    pub fn new(pool: SqlitePool) -> Self {
        Self(pool)
    }
}
