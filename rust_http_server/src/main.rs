#![allow(dead_code)]

use server::Server;
use http::Method;
use http::Request;

mod server;
mod http;

fn main() { 
    let ip = "127.0.0.1:8080".to_string();
    let server = Server::new(ip);
    server.run();
}