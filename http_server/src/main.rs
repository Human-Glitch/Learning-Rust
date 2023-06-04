use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;

fn main() {
    let _get = Method::GET("abcd".to_string());
    let _delete = Method::DELETE;
    let _post = Method::POST(100);
    let _put = Method::PUT;

    let _string = String::from("127.0.0.1:8080".to_string());
    let _server = Server::new(_string);
    _server.run()
}



