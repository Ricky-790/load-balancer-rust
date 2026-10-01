use std::{
    // alloc::System,
    // fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();
    let mut host_name: Option<String> = None;
    let mut request_line: Option<String> = None;

    loop {
        match lines.next() {
            Some(Ok(line)) => {
                if line.is_empty() {
                    break;
                }
                if line.starts_with("Host") || line.starts_with("host") {
                    if let Some((_, val)) = line.split_once(":") {
                        host_name = Some(val.trim().to_string())
                    }
                }
                if line.starts_with("GET") || line.starts_with("POST") {
                    request_line = Some(line);
                }
            }

            Some(Err(_)) | None => break,
        }
    }
    println!("{:?} {:?}", host_name, request_line)
}
