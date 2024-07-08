use std::{future::Future, pin::Pin};

use hyper::Request;
use tower::Service;


#[derive(Debug, Clone)]
pub struct Logger<S> {
    inner: S
}

impl<S> Logger<S> {
    pub fn new(inner: S) -> Self {
        Logger { inner }
    }
}


impl<S, B> Service<Request<B>> for Logger<S> 
where B: 'static + Send,
      S: Service<Request<B>> + Clone + 'static + Send,
      S::Future: Send
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static + Send>> ;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let method = req.method().clone();
            let uri = req.uri().clone();
            println!("{} {}", method, uri);
            let fut = inner.call(req).await;
            println!("{} {}", method, uri.path());
            return fut;
        })
    }
}