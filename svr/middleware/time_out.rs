use std::{fmt, marker};

use ntex::time::{sleep, Millis};
use ntex::util::timeout::TimeoutError;
use ntex::util::{select, Either};

use ntex::web::WebRequest;
use ntex::{web, Middleware};
use ntex::{Service, ServiceCtx};

use crate::svr::error::HttpResError;

#[derive(Debug)]
pub struct Timeout<E = ()> {
    timeout: Millis,
    _t: marker::PhantomData<E>,
}

impl Timeout {
    pub fn new<T: Into<Millis>>(timeout: T) -> Self {
        Timeout { timeout: timeout.into(), _t: marker::PhantomData }
    }
}

impl<S> Middleware<S> for Timeout {
    type Service = TimeoutService<S>;

    fn create(&self, service: S) -> Self::Service {
        TimeoutService { service, timeout: self.timeout }
    }
}

#[derive(Debug, Clone)]
pub struct TimeoutService<S> {
    service: S,
    timeout: Millis,
}

impl<S, Err> Service<web::WebRequest<Err>> for TimeoutService<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_ready!(service);
    ntex::forward_shutdown!(service);

    async fn call(&self, req: WebRequest<Err>, ctx: ServiceCtx<'_, Self>) -> Result<Self::Response, Self::Error> {
        if self.timeout.is_zero() {
            ctx.call(&self.service, req).await
        } else {
            match select(sleep(self.timeout), ctx.call(&self.service, req)).await {
                Either::Left(_) => Err(web::Error::new(HttpResError::NtexWeb(web::error::ErrorGatewayTimeout("111").into()))),
                Either::Right(res) => res,
            }
        }
    }
}
