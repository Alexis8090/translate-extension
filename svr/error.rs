use axum::{
  extract::rejection::QueryRejection, http::StatusCode, response::{IntoResponse, Response}
};

// use deadpool_sqlite::{InteractError, PoolError};
use tokio_rusqlite_folk::{Error};
// use deadpool_sqlite::{rusqlite, InteractError, PoolError};
use serde::{  Serialize};
use thiserror::Error; // 1. 导入 thiserro
use validator::ValidationErrors;





#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResErrorResponse {
  msg: String,
}

#[derive(Error, Debug)]
pub enum HttpResError {

    #[error("Bad Request: {0:?}")] // {0} 会被替换为 String 的内容
    SerdeJSON( #[from] serde_json::Error),

    #[error("Bad Request: {0:?}")] // {0} 会被替换为 String 的内容
    BadRequest( String),

    #[error("Validation failed: {0:?}")] // {0} 会被替换为 String 的内容
    HttpValidation(#[from] ValidationErrors),


    #[error("A database operation failed  :{0:?}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Failed to connect to the database service")]
    TokioRusqlite(#[from] Error),

    // #[error("Failed to connect to the database service")]
    // DeadPool(#[from] PoolError),

    // #[error("An internal error occurred while processing the request")]
    // DeadPoolInteract(#[from] InteractError),



}





impl IntoResponse for HttpResError {
    fn into_response(self) -> Response {


        let msg = self.to_string();
        let res = match &self { // 注意这里用 &self，因为 self 后面可能还会被用于日志记录



            HttpResError::SerdeJSON(source_err) => {
                // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database error encountered");
                (StatusCode::INTERNAL_SERVER_ERROR,msg)
            }

            HttpResError::BadRequest(source_err) => {
                // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database error encountered");
                (StatusCode::BAD_REQUEST,msg)
            }

            HttpResError::HttpValidation(source_err) => {
                // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database error encountered");
                (StatusCode::BAD_REQUEST,msg)
            }
            HttpResError::Sqlite(source_err) => {
                // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database error encountered");
                (StatusCode::BAD_REQUEST,msg)//"dataQL error".to_owned())
            }
            HttpResError::TokioRusqlite(source_err) => {
                // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database pool error encountered");
                (StatusCode::INTERNAL_SERVER_ERROR,msg)
            }
            // HttpResError::DeadPoolInteract(source_err) => {
            //     // tracing::error!(error.message = %error_message, source.error = ?source_err, "Database interact task error encountered");
            //     (StatusCode::INTERNAL_SERVER_ERROR,msg)
            // }
        };
        res.into_response()
    }
}

