use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;

fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let string = String::from("127.0.0.1:8080".to_string());
    let server = Server::new(string);
    server.run()
}



