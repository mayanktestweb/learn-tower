use std::{future::Future, io::Error, task::Poll};

use tower::Service;

pub struct MyFuture {
    value: String
}


impl Future for MyFuture {
    type Output = Result<u32, Error>;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self.value.len() as u32))
    }
}


pub struct BasicService;


// Let;s say request type for this Service is String 
// in this implementation...
impl Service<String> for BasicService {
    type Response = u32;

    type Error = Error;

    type Future = MyFuture;   // this should implement Future trait

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        MyFuture { value: req}
    }
}