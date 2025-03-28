use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn get_file_from_status(status: &str) -> (&'static str, &'static str) {
    let request = status
        .split_whitespace()
        .find(|&s| s.starts_with("/"))
        .unwrap_or("");

    match request {
        "/" => ("HTTP/1.1 200 OK", "html/hello.html"),
        "/page1" => ("HTTP/1.1 200 OK", "html/page1.html"),
        "/page1/page2" => ("HTTP/1.1 200 OK", "html/page2.html"),
        "/page3" => ("HTTP/1.1 200 OK", "html/page3.html"),
        _ => ("HTTP/1.1 200 OK", "html/404.html"),
    }
}

const ADDRESS: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let (status_line, filename) = get_file_from_status(&http_request[0]);

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    let l = &http_request[0];
    println!("{l:#?}");
    stream.write_all(response.as_bytes()).unwrap();
}
