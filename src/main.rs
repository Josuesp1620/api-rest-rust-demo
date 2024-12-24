mod core;
mod docker;
use crate::core::server;

fn main() {
    let host = "0.0.0.0";
    let port = 8081;

    let server = server::Server::new(host, port);
    
    server.listen().unwrap();
}