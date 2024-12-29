mod core;
mod docker;
mod odoo;
use crate::core::server;

fn main() {
    let host = "0.0.0.0";
    let port = 7575;

    let server = server::Server::new(host, port);
    
    match server.listen() {
        Ok(_) => {
            println!("El servidor se ha iniciado correctamente en {}:{}", host, port);
        }
        Err(e) => {
            eprintln!("Error al iniciar el servidor");
            eprintln!("{}", e);
        }
    };
}