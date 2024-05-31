use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};

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
}
