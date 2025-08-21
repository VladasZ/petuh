
#![allow(dead_code)]

use anyhow::Result;

pub struct Model;

impl Model {
    pub fn tables() -> &'static [&'static str] {
        &[
            "chats",
            "saved_responses",
            "users",
            "user_stats",
        ]
    }

    pub async fn drop_all_tables(pool: &sqlx::PgPool) -> Result<()> {
        use sqlx::Executor;

        for table in Self::tables() {
            pool.execute(&*format!("DROP TABLE IF EXISTS {table} CASCADE;")).await?;
        }

        Ok(())
    }
}
        