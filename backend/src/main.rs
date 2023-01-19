use std::{io::prelude::*, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let status_line = "HTTP/1.1 200 OK";
        let contents = "Hello, World!";
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
