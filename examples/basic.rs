fn main() {
    let mut app = mp::Server::new(5000);

    app.get("/", || println!("Sike ra babu"));

    app.serve();
}
