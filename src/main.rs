
use std::{error::Error, net::SocketAddr};

use hyper::server::conn::http1;
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use services::{http_service::BasicHttpService, logger::Logger};
use tokio::net::TcpListener;

pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);
        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // let's first make tower service
            let hello = BasicHttpService;
            let hello = Logger::new(hello);
            let hello = TowerToHyperService::new(hello);
            
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, hello)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
