use reqwest::StatusCode;

pub fn gmoney() -> Result<String, (StatusCode, &'static str)> {
    Ok("get /test2/client/money path is ok. ".into())
}

pub fn pmoney() -> Result<String, (StatusCode, &'static str)> {
    Ok("post /test2/client/money path is ok. ".into())
}

pub fn gclient() -> Result<String, (StatusCode, &'static str)> {
    Ok("get /test2/client/ path is ok. ".into())
}

pub fn pclient() -> Result<String, (StatusCode, &'static str)> {
    Ok("post /test2/client/ path is ok. ".into())
}
