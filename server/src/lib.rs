use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use database::Database;

const ADDRESS: &str = "127.0.0.1:7878";

#[derive(Default)]
pub struct Server {
    pub database: Database,
}

impl Server {
    pub fn new() -> Self {
        let database = Database::new();
        Self { database }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(ADDRESS).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }
    ///converts the requested url to a file path
    fn get_file_from_status(&self, status: &str) -> (&'static str, &str) {
        let request = status
            .split_whitespace()
            .find(|&s| s.starts_with("/") && s.ne("/favicon.ico"));

        let status = if request.is_some() {
            "HTTP/1.1 200 404 NOT FOUND"
        } else {
            "HTTP/1.1 200 OK"
        };
        eprintln!("{request:#?}");

        (status, self.database.get(request.unwrap_or("")))
    }
    ///reads a tcpstream and returns the webpage requested
    ///currently only returns a html file
    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let (status_line, filename) = self.get_file_from_status(&http_request[0]);
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
