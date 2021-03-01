use std::{
    io::{Read, Write},
    net::TcpListener,
};

use std::sync::{Arc, Mutex, RwLock};

use std::collections::HashMap;

pub mod pool;
pub mod request;
pub mod response;

pub struct RequestContext {
    pub headers: HashMap<String, String>,
}

pub struct Server {
    pub port: u16,
    get_handlers: HashMap<String, Box<dyn Fn(request::Request, response::Response)>>,
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

pub struct Router {
    pub get_handlers:
        HashMap<String, Box<dyn Fn(request::Request, response::Response) + Sync + Send + 'static>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            get_handlers: HashMap::new(),
        }
    }

    pub fn get<T>(&mut self, path: &str, handler: T)
    where
        T: Fn(request::Request, response::Response) + Send + Sync + 'static,
    {
        self.get_handlers
            .insert(path.to_string(), Box::new(handler));
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
    pub fn serve(&mut self, router: Router) {
        self.run(router);
    }

    fn run(&mut self, router: Router) {
        let tpool = pool::Pool::new(32);

        let server = match TcpListener::bind(format!("localhost:{}", self.port)) {
            Ok(s) => s,
            Err(e) => panic!("Failed to bind: {}", e),
        };

        let router_mut = Arc::new(RwLock::new(router));

        for socket in server.incoming() {
            let socket = socket.unwrap();
            let router = Arc::clone(&router_mut);
            //handle_conn(socket, router);
            tpool.execute(move || handle_conn(socket, router));
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
        T: Fn(request::Request, response::Response) + Send + Sync + 'static,
    {
        self.get_handlers
            .insert(path.to_string(), Box::new(handler));
    }

    pub fn post() {}

    pub fn delete() {}

    pub fn put() {}

    pub fn patch() {}
}

fn handle_conn(mut s: std::net::TcpStream, router: Arc<RwLock<Router>>) {
    let mut buf = [0; 1024];
    match s.read(&mut buf) {
        Ok(s) => {
            //println!("[Req] : Read {} bytes", s);
            if s == 0 {
                return;
            }
        }
        Err(e) => {
            println!("Failed to read: {}", e)
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

    let req = request::Request::new(headers);
    let method = req.headers.get("method").unwrap();
    let resp = response::Response::new(s);

    match HTTPMethods::to_http(&method) {
        HTTPMethods::GET => {
            if req.headers.get("path").unwrap() == "/favicon.ico" {
                return;
            }
            let locked_router = router.read().unwrap();
            let hand = locked_router
                .get_handlers
                .get(req.headers.get("path").unwrap())
                .unwrap();

            hand(req, resp);
        }
        HTTPMethods::POST => {
            println!("{:?}", req.headers);
        }
        HTTPMethods::DELETE => {}
        HTTPMethods::PATCH => {}
        HTTPMethods::PUT => {}
    };
}
