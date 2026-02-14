use futures::stream::StreamExt;
use futures::stream::{self, Stream};
use ntex::http::body::{Body, BodySize, MessageBody, ResponseBody};
use ntex::http::{Payload, PayloadStream};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::util::{Bytes, BytesMut};
use ntex::web::error::PayloadError;
use ntex::web::{Error, ErrorRenderer, WebRequest, WebResponse};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, fmt::Display};

// A helper function to convert BytesMut into a stream of Bytes chunks.
fn bytes_mut_to_stream(body: &Bytes) -> PayloadStream {
    let bytes_clone = Bytes::from(body);
    Box::pin(stream::once(async move { Ok(bytes_clone) }))
}

pub struct Logging;

impl<S> Middleware<S> for Logging {
    type Service = LoggingMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        LoggingMiddleware { service }
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for LoggingMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error> + 'static,
    Err: ErrorRenderer + 'static,
{
    type Response = WebResponse;
    type Error = Error;

    ntex::forward_ready!(service);
    ntex::forward_shutdown!(service);

    async fn call(&self, mut req: WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        let mut request_body_bytes = BytesMut::new();
        let mut stream = req.take_payload();
        while let Some(chunk) = stream.next().await {
            request_body_bytes.extend_from_slice(&chunk?);
        }
        let request_body_bytes = request_body_bytes.freeze();
        let stream: PayloadStream = bytes_mut_to_stream(&request_body_bytes);
        req.set_payload(Payload::from(stream));
        println!("{:?}",request_body_bytes);
        ctx.call(&self.service, req).await//.map(|res| res.map_body(move |_, body| Body::from_message(BodyLogger { body, request_body_bytes, response_body_bytes: BytesMut::new() }).into()))
    }
}

pub struct BodyLogger {
    body: ResponseBody<Body>,
    request_body_bytes: Bytes,
    response_body_bytes: BytesMut,
}

impl Drop for BodyLogger {
    fn drop(&mut self) {
        println!("{:?}-{:?}", self.request_body_bytes, self.response_body_bytes);
    }
}

impl MessageBody for BodyLogger {
    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next_chunk(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, Box<dyn std::error::Error>>>> {
        match self.body.poll_next_chunk(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                self.response_body_bytes.extend_from_slice(&chunk);
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
