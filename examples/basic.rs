fn main() {
    let mut app = mp::Server::new(5000);

    let mut router = mp::Router::new();

    router.get("/", |req, mut res| {
        res.body("<h1>hello world</h1>");
    });

    router.get("/json", |req, mut res| {
        res.add_header("Content-Type", "application/json");
        res.body("{ \"name\" : \"dave\"  }");
    });

    router.get("/about", |req, mut res| {
        res.body("<p>simple about page</p>")
    });

    app.serve(router);
}
