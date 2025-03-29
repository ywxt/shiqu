use sqlx::{Row, SqlitePool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Character {
    pub id: i32,
    pub name: String,
}

pub struct CharacterContext(SqlitePool);
impl CharacterContext {
    pub const TABLE_NAME: &'static str = "characters";
    pub const ID: &'static str = "id";
    pub const NAME: &'static str = "name";

    pub fn new(pool: SqlitePool) -> CharacterContext {
        CharacterContext(pool)
    }

    pub async fn find_by_id(&mut self, id: i64) -> Result<Option<Character>, sqlx::Error> {
        let mut conn = self.0.acquire().await?;
        let row = sqlx::query("SELECT ({}, {}) FROM {} WHERE {} = ?")
            .bind(Self::ID)
            .bind(Self::NAME)
            .bind(Self::TABLE_NAME)
            .bind(Self::ID)
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(row.map(|r| Character {
            id: r.get(Self::ID),
            name: r.get(Self::NAME),
        }))
    }
}
