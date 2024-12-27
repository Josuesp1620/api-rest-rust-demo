mod core;
mod docker;
mod odoo;
use crate::core::server;

fn main() {
    let host = "0.0.0.0";
    let port = 7575;

    let server = server::Server::new(host, port);
    
    server.listen().unwrap();
}