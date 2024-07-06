
use tower::Service;

pub mod services;
use services::BasicService;

#[tokio::main]
async fn main() {

    let mut bs = BasicService;

    let fut = bs.call(String::from("Lallu Ram!"));

    let ans = fut.await.unwrap();

    println!("MyFuture fut resolves to Result with value {ans}");
}
