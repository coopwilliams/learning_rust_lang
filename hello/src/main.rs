use std::{
    net::{TcpStream, TcpListener}, 
    io::{prelude::*, BufReader}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // The browser signals the end of a request by sending \n\n
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // write_all() takes &[u8] and sends those bytes down the connection.
    stream.write_all(response.as_bytes()).unwrap();
}