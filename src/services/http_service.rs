use std::{convert::Infallible, future::Future, task::Poll};

use hyper::{body::Incoming, Request, Response};
use tower::Service;

#[derive(Debug)]
pub struct BasicHttpFuture {
    req: Request<Incoming>
}

impl Future for BasicHttpFuture {
    type Output = Result<Response<String>, Infallible>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Poll::Ready(Ok(Response::new(String::from("Hello, World!"))))
    }
}

#[derive(Debug, Clone)]
pub struct BasicHttpService;

impl Service<Request<Incoming>> for BasicHttpService {
    type Response = Response<String>;

    type Error = Infallible;

    type Future = BasicHttpFuture;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        BasicHttpFuture {req}
    }
}