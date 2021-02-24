use std::{
    io::{Read, Write},
    net::TcpListener,
};

use std::collections::HashMap;

pub struct RequestContext {
    pub headers: HashMap<String, String>,
}

pub struct Server {
    pub port: u16,
    get_handlers: HashMap<String, Box<dyn Fn()>>,
}

#[derive(Debug)]
pub struct RLine {
    pub path: String,
    pub method: String,
}

pub enum HTTPMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HTTPMethods {
    /// Convert method of string into an enum.
    pub fn to_http(method: &str) -> Self {
        match method {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "PATCH" => Self::PATCH,
            _ => Self::GET,
        }
    }
}

impl Server {
    /// Create a new server instance.
    /// # Example
    /// ```
    ///
    ///let app = mp::Server::new(5000);
    ///
    /// ```
    pub fn new(port: u16) -> Self {
        Server {
            port,
            get_handlers: HashMap::new(),
        }
    }

    /// Start listening to connections on given port.
    pub fn serve(&mut self) {
        self.run();
    }

    fn run(&mut self) {
        let server = match TcpListener::bind(format!("localhost:{}", self.port)) {
            Ok(s) => s,
            Err(e) => panic!("Failed to bind: {}", e),
        };

        for socket in server.incoming() {
            if let Ok(mut s) = socket {
                let mut buf = [0; 1024];
                match s.read(&mut buf) {
                    Ok(s) => {
                        if cfg!(feature = "log") {
                            println!("[Req] : Read {} bytes", s)
                        }
                    }
                    Err(e) => {
                        if cfg!(feature = "log") {
                            println!("Failed to read: {}", e)
                        }
                    }
                };
                let req_body = String::from_utf8_lossy(&buf);

                let mut headers = HashMap::new();

                for (idx, line) in req_body.lines().enumerate() {
                    if idx == 0 {
                        let req_line: Vec<&str> = line.split(" ").collect();
                        headers.insert("method".to_string(), req_line[0].to_string());
                        headers.insert("path".to_string(), req_line[1].to_string());
                    } else {
                        let parts: Vec<&str> = line.split(": ").collect();
                        if parts.len() > 1 {
                            headers.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                }

                println!("{:?}", headers);

                let method = headers.get("method").unwrap();

                match HTTPMethods::to_http(&method) {
                    HTTPMethods::GET => {
                        let handler = self.get_handlers.get(headers.get("path").unwrap());
                        handler.unwrap()();
                    }
                    HTTPMethods::POST => {}
                    HTTPMethods::DELETE => {}
                    HTTPMethods::PATCH => {}
                    HTTPMethods::PUT => {}
                };

                let ctx = RequestContext { headers };

                let mut resp = String::new();
                resp.push_str("HTTP/1.1 200 OK\r\n\r\n<h1>sike</h1>");

                let written = s.write(resp.as_bytes()).unwrap();

                println!("Written {} bytes", written)
            }
        }
    }

    pub fn has_route(&self, path: &str) -> bool {
        self.get_handlers.contains_key(path)
    }

    /// Register a get request that is to be handled for a path.
    /// # Example
    /// ```
    /// use mp::Server;
    ///
    /// let mut app = Server::new(5000);
    /// app.get("/",|| {});
    /// assert!(app.has_route("/"))
    ///
    /// ```
    pub fn get<T>(&mut self, path: &str, handler: T)
    where
        T: Fn() + Send + 'static,
    {
        self.get_handlers
            .insert(path.to_string(), Box::new(handler));
    }

    pub fn post() {}

    pub fn delete() {}

    pub fn put() {}

    pub fn patch() {}
}
