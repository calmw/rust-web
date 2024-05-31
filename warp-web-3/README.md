## 功能

- 添加数据库查询Sqlx

## 数据库迁移

- 安装 Sqlx cli
    - ``` cargo install sqlx-cli ```
- 创建第一个迁移
    - ``` sqlx migrate add -r questions_table ```

- 执行迁移 

``` shell
sqlx migrate run --database-url postgres://root:root@localhost:5432/dbname ```

# 通过代码执行迁移

let store = store::Store::new("postgres://localhost:5432/rustwebdev").await;
sqlx::migrate!()
.run(&store.clone().connection)
.await
.expect("Cannot migrate DB");

```

- 撤销改动
    - ``` sqlx migrate revert --database-url postgres://root:root@localhost:5432/dbname ```
    - 每次撤销都会触发最新的迁移，并尝试运行*.down.sql