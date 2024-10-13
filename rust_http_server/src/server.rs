use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));
                            
                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {},
                                Err(e) => { println!("Failed to parse a request: {}", e)}
                            }

                            //let res: &Result<Request, _> = &buffer[..];

                        },
                        Err(e) => {}
                    } 
                },
                Err(e) => {
                    println!("failed to establish a connection {}", e);
                }
            }

            let res = listener.accept();
        }
    }
}