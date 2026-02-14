

use deadpool_sqlite::{Config, Manager, Pool, Runtime};
use rusqlite::params;




// fn get_config() -> AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>> {
//     let  = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(env::var("sqlite_url"));
//     config
// }

// pub async fn db_conn<DB: Database>(url: &str) -> Result<Pool<DB>> {
//     let db =
//     Ok(db)
// }


//  async fn db_conn(db_url:impl Into<String>) -> Pool<AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>>> {

//        let config = AsyncDieselConnectionManager::<SyncConnectionWrapper::<SqliteConnection>>::new(db_url);

//     // #[cfg(feature = "postgres")]
//     // let pool: Pool<AsyncPgConnection> = Pool::builder(config).build()?;
//     // #[cfg(not(feature = "postgres"))]


//     Pool::builder(config).build().unwrap()
//  }


pub async fn db_conn(db_url: &str) -> Pool {
    let cfg = Config::new(db_url);
    let pool = cfg.create_pool(Runtime::Tokio1).unwrap();
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| {


        conn.execute_batch("
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL; /* 或 OFF，如果你敢 */
                PRAGMA busy_timeout = 50000;
                PRAGMA cache_size = -200001; /* 约 200MB */
                PRAGMA temp_store = MEMORY;
                PRAGMA default_transaction_mode = IMMEDIATE;
                PRAGMA foreign_keys = OFF;
                PRAGMA mmap_size = 268435456; /* 256MB, 谨慎测试 */
                PRAGMA wal_autocheckpoint = 5005;
                PRAGMA page_size = 8192;


                -- 检查是否存在 user 表，如果存在则删除它
                DROP TABLE IF EXISTS user;
                    -- 创建新的 user 表
                CREATE TABLE user (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL  COLLATE NOCASE, -- 大小写不敏感的唯一约束
                    age INTEGER NOT NULL,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    updated_at DATETIME DEFAULT NULL,
                    deleted_at DATETIME DEFAULT NULL
                 );
                 CREATE INDEX user_deleted_at_age_name_id_1747242058824 ON user ('deleted_at','age','name','id');
            ").unwrap();
            let busy_timeout:u16 = conn.pragma_query_value(None, "busy_timeout", |row| row.get(0)).unwrap();
            dbg!(busy_timeout);
            let cache_size:i32 = conn.pragma_query_value(None, "cache_size", |row| row.get(0)).unwrap();
            dbg!(cache_size)

    }).await.unwrap();
    dbg!("db is ready");
    pool
}



// pub async fn db_conn1(db_url: &str) -> Pool {
//     let cfg = Config::new(&db_url);
//     let pool = cfg.create_pool(Runtime::Tokio1).unwrap();
//     let conn = pool.get().await.unwrap();
//     conn.interact(|conn| {
//             conn.execute_batch(
//                 "PRAGMA journal_mode = WAL;
//                         PRAGMA synchronous = NORMAL; /* 或 OFF，如果你敢 */
//                         PRAGMA busy_timeout = 30000;
//                         PRAGMA cache_size = -200000; /* 约 200MB */
//                         PRAGMA temp_store = MEMORY;

//                         /* 可选 */
//                         PRAGMA foreign_keys = OFF;
//                         PRAGMA mmap_size = 268435456; /* 256MB, 谨慎测试 */
//                         PRAGMA wal_autocheckpoint = 4000;
//                         PRAGMA page_size = 8192;

//                         ").unwrap();
//     }).await.unwrap();
//     // conn.execute_batch(
//     //     "PRAGMA journal_mode = WAL;
//     //      PRAGMA synchronous = NORMAL; /* 或 OFF，如果你敢 */
//     //      PRAGMA busy_timeout = 30000;
//     //      PRAGMA cache_size = -200000; /* 约 200MB */
//     //      PRAGMA temp_store = MEMORY;"
//     // )?;
//     pool

// }
