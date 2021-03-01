use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Response {
    sock: TcpStream,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(sock: TcpStream) -> Self {
        Response {
            sock,
            headers: HashMap::new(),
        }
    }

    pub fn body(&mut self, text: &str) {
        let mut resp_string = String::new();
        resp_string.push_str("HTTP/1.1 200 OK\r\n");
        if self.headers.len() > 0 {
            for (k, v) in &self.headers {
                resp_string.push_str(&format!("{}: {}\r\n", k, v))
            }
        }
        resp_string.push_str("\r\n");
        resp_string.push_str(text);

        let written = self.sock.write(resp_string.as_bytes()).unwrap();
        println!("{}", written);
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}
