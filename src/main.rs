use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8564").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //  针对每一个连接建立一个线程
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";


    let (status_line, fine_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(fine_name).unwrap();
    let response = format!("{}{}", status_line, contents);

    // 写入响应
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}