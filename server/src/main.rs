use server::Server;

fn main() {
    let mut s = Server::new();
    let _ = s.database.add("example");
    s.run();
}
