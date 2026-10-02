use std::{
    io::{self, BufReader, prelude::*},
    net::{Shutdown, TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut server_no: u32 = 8080;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut server_no);
        println!("{}", server_no)
    }
}

fn handle_connection(mut stream: TcpStream, server_no: &mut u32) {
    *server_no += 1;
    if *server_no == 8084 {
        *server_no = 8080;
    }

    let mut server_stream = match TcpStream::connect(format!("127.0.0.1:{}", server_no)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to backend server {}: {}", server_no, e);
            return;
        }
    };
    println!("Successfully connected to server {}!", server_no);

    // 1. Forward the request from Browser -> Backend
    // Clone stream to read request bytes, or use try_clone
    let mut client_reader = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return;
        }
    };

    // Forward request
    if let Err(e) = io::copy(&mut client_reader, &mut server_stream) {
        eprintln!("Error sending request to server: {}", e);
        return;
    }

    // 2. CRITICAL: Signal to backend that request sending is finished!
    let _ = server_stream.shutdown(Shutdown::Write);

    // 3. Forward the response from Backend -> Browser
    match io::copy(&mut server_stream, &mut stream) {
        Ok(bytes) => println!("Sent {} response bytes back to browser", bytes),
        Err(e) => eprintln!("Error sending response to client: {}", e),
    }
}
