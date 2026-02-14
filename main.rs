
mod svr;
mod api;
mod biz;
mod server;

use api::{dmail::build_dmail_routes};
use axum::{extract::rejection::QueryRejection, Router};
// use deadpool_sqlite::Pool;

// use deadpool_sqlite::Pool;
// use diesel::{sqlite::SqliteBindValue, Connection, SqliteConnection};
// use diesel_async::{async_connection_wrapper::AsyncConnectionWrapper, sync_connection_wrapper::SyncConnectionWrapper, AsyncConnection,As };
use dotenvy::{ from_filename};
// use svr::{ db::db_conn};
use tokio::sync::{mpsc::Sender, Mutex};
use tokio_rusqlite_folk::Connection;
use tracing::Level;
use tracing_subscriber::fmt;
use std::{ env, net::SocketAddr, sync::{Arc, OnceLock}};
// use server;

#[derive(Clone)]
pub struct AppState {
    pub dmail_db: Connection,
}

use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use crate::svr::error::HttpResError;

async fn serve_app() {

    let dmail_db = Connection::open(&env::var("db_name1").unwrap()).await.unwrap();
    dmail_db.call(|conn|{
        conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL; /* 或 OFF，如果你敢 */
        PRAGMA busy_timeout = 50000;
        PRAGMA cache_size = -200001; /* 约 200MB */
        PRAGMA temp_store = MEMORY;
        PRAGMA default_transaction_mode = IMMEDIATE;
        PRAGMA foreign_keys = OFF;
        PRAGMA mmap_size = 3000000000; /* 256MB, 谨慎测试 */
        PRAGMA wal_autocheckpoint = 5005;
        PRAGMA page_size = 32768;
    ").unwrap();
        let busy_timeout:u16 = conn.pragma_query_value(None, "busy_timeout", |row| row.get(0)).unwrap();
        let cache_size:i32 = conn.pragma_query_value(None, "cache_size", |row| row.get(0)).unwrap();
        println!("busy_timeout:${busy_timeout}");
        println!("busy_timeout:${busy_timeout}");
        Ok(())
    }).await.unwrap();





    // let app = Router::new()
    //     .route("/fortunes", get(fortunes))
    //     .route("/db", get(db))
    //     .route("/queries", get(queries))
    //     .route("/updates", get(updates))
    //     .with_state(conn);


    let app = Router::new()
    .merge(build_dmail_routes())
    .with_state(AppState {dmail_db});

    server::serve(app, Some(env::var("port").unwrap().parse::<u16>().unwrap())).await
}

fn main() {
    from_filename(".env").ok();
    server::start_tokio(serve_app)
}