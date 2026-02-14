use redis::AsyncCommands;
use reqwest::StatusCode;
use tracing::info;

use crate::global::{get_mut, REDIS1};

pub fn gmoney() -> Result<String, (StatusCode, &'static str)> {
    Ok("get /test2/server/money path is ok. ".into())
}

pub fn pmoney() -> Result<String, (StatusCode, &'static str)> {
    Ok("post /test2/server/money path is ok. ".into())
}

pub async fn gserver() -> Result<String, (StatusCode, &'static str)> {
    // let value: String = unsafe { get_mut(&mut REDIS1).unwrap().get("妈妈").await.unwrap() };
    let value: String = unsafe { get_mut(&mut REDIS1).unwrap().get("妈妈").await.unwrap() };
    info!(value);
    Ok("get /test2/server/ path is ok. ".into())
}

pub async fn pserver() -> Result<String, (StatusCode, &'static str)> {
    unsafe {
        get_mut(&mut REDIS1)
            .unwrap()
            .set::<&str, &str, ()>("妈妈", "111")
            .await
            .unwrap();
    };
    Ok("post /test2/server/ path is ok. ".into())
}
