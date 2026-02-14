use redis::{aio::ConnectionManager};

pub async fn redis_conn(redis_url: &str) -> ConnectionManager {
    let client = redis::Client::open(redis_url).unwrap();
    ConnectionManager::new(client).await.unwrap()
}
