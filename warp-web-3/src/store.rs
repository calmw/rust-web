use handle_errors::Error;
use sqlx::postgres::{PgPoolOptions, PgPool, PgRow, PgDatabaseError};
use sqlx::Row;
use warp::Filter;
use crate::types::{
    question::Question,
    question::QuestionId,
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool, // 设置连接池
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new().max_connections(100)
            .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("couldn't establish DB connection:{}", e) // 如果无法建立数据库连接，会让应用启动失败
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("select * from questions limit $1 offset $2").bind(limit).bind(offset).map(
            |row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            }
        ).fetch_all(&self.connection).await {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR,"{:?}",e);
                Err(e)
            }
        }
    }
}
