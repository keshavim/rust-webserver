use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use database::Database;
use thread_pool::ThreadPool;

const ADDRESS: &str = "127.0.0.1:8080";

pub fn run(address: Option<&str>) {
    let address = address.unwrap_or(ADDRESS);
    let listener = TcpListener::bind(address).unwrap();
    let database = Arc::new(Database::load());

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let db_clone = Arc::clone(&database);
        pool.execute(|| {
            handle_connection(stream, db_clone);
        });
    }
}
///converts the requested url to a file path
fn get_file_from_status<'a>(status: &str, database: &'a Database) -> (&'static str, &'a str) {
    let request = status
        .split_whitespace()
        .find(|&s| s.starts_with("/") && s.ne("/favicon.ico"));

    let status = if request.is_some() {
        "HTTP/1.1 200 404 NOT FOUND"
    } else {
        "HTTP/1.1 200 OK"
    };
    let request = request.unwrap_or("");

    (status, database.get(request))
}
///reads a tcpstream and returns the webpage requested
///currently only returns a html file
fn handle_connection(mut stream: TcpStream, database: Arc<Database>) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let (status_line, filename) = get_file_from_status(&http_request[0], &database);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
