use std::path::Path;

use sqlx::{
    SqliteConnection, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::entity::{CharacterContext, ConversationContext, MessageContext, UserContext};

pub async fn connect(path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    let opt = SqliteConnectOptions::new()
        .filename(path)
        .read_only(false)
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(opt)
        .await
}

pub async fn close(db: SqlitePool) {
    db.close().await
}

pub async fn try_create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut db = pool.acquire().await?;
    create_user_table(&mut db).await?;
    create_character_table(&mut db).await?;
    create_conversation_table(&mut db).await?;
    create_message_table(&mut db).await?;
    Ok(())
}

async fn create_user_table(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(r#"
        CREATE TABLE IF NOT EXISTS `{}` (
            `{}` INTEGER PRIMARY KEY AUTOINCREMENT,
            `{}` BIGINT UNSIGNED NOT NULL,
            `{}` VARCHAR(100) NOT NULL
        );
        "#,
        UserContext::TABLE_NAME,
        UserContext::ID,
        UserContext::TELEGRAM_ID,
        UserContext::NAME,
    ))
    .execute(conn)
    .await?;

    Ok(())
}

async fn create_character_table(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        &format!(r#"
        CREATE TABLE IF NOT EXISTS {} (
            {} INTEGER PRIMARY KEY AUTOINCREMENT,
            {} VARCHAR(100) NOT NULL
        );
        "#,
            CharacterContext::TABLE_NAME,
            CharacterContext::ID,
            CharacterContext::NAME,
        )
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn create_conversation_table(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        &format!(r#"
        CREATE TABLE IF NOT EXISTS {} (
            {} INTEGER PRIMARY KEY AUTOINCREMENT,
            {} INTEGER NOT NULL,
            {} INTEGER NOT NULL,
            FOREIGN KEY ({}) REFERENCES {}({}),
            FOREIGN KEY ({}) REFERENCES {}({})
        );
        "#,
            ConversationContext::TABLE_NAME,
            ConversationContext::ID,
            ConversationContext::USER_ID,
            ConversationContext::CHARACTER_ID,
            ConversationContext::USER_ID,
            UserContext::TABLE_NAME,
            UserContext::ID,
            ConversationContext::CHARACTER_ID,
            CharacterContext::TABLE_NAME,
            CharacterContext::ID,
        )
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn create_message_table(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        &format!(r#"
        CREATE TABLE IF NOT EXISTS {} (
            {} INTEGER PRIMARY KEY AUTOINCREMENT,
            {} INTEGER NOT NULL,
            {} INTEGER NOT NULL,
            {} TEXT NOT NULL,
            {} TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE CASCADE
        );
        "#,
            MessageContext::TABLE_NAME,
            MessageContext::ID,
            MessageContext::CONVERSATION_ID,
            MessageContext::ROLE,
            MessageContext::CONTENT,
            MessageContext::CREATED_AT,
            MessageContext::CONVERSATION_ID,
            ConversationContext::TABLE_NAME,
            ConversationContext::ID,
        )
    )
    .execute(conn)
    .await?;

    Ok(())
}
