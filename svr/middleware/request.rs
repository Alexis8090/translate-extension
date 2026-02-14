use axum::{
    async_trait,
    extract::{FromRequestParts, Query}, // Assuming Query for example
    http::request::Parts,
    RequestPartsExt,
};
use serde::de::DeserializeOwned;
use std::fmt::Display;
use crate::svr::error::HttpResError; // Your HttpResError

// Assume HttpResError has a variant like this:
// enum HttpResError {
//     DeserializationError(String),
//     // ...
// }
// impl Display for HttpResError { ... }
// impl std::error::Error for HttpResError { ... }


// This extractor wraps another extractor (e.g., Query) and converts its rejection
pub struct MyWrapperExtractor<P>(pub P);

#[async_trait]
impl<S, P> FromRequestParts<S> for MyWrapperExtractor<P>
where
    S: Send + Sync,
    P: DeserializeOwned, // P is the type that uses Serde (e.g., UserPayload)
{
    type Rejection = HttpResError; // The final rejection type

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Use an underlying Axum extractor that performs Serde deserialization
        match Query::<P>::from_request_parts(parts, state).await {
            Ok(Query(payload)) => Ok(MyWrapperExtractor(payload)),
            Err(rejection) => {
                // `rejection` is QueryRejection.
                // If UserPayload.age caused string_to_number to fail with de::Error::custom(HttpResError{msg: "X"}),
                // then rejection.to_string() will be something like "Failed to deserialize query string: X"
                Err(HttpResError::DeserializationError(rejection.to_string()))
            }
        }
    }
}

