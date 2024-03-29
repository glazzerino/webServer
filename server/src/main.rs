use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
mod threadpool;
const LOCAL: &str = "127.0.0.1:7878";
fn main() {
    let threads = threadpool::Threadpool::new(4);
    let listener = TcpListener::bind(LOCAL)
    .expect("Could not bind to LOCAL");
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        threads.execute(|| {handle_connection(stream)});
    }   
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
     
    } else if buffer.starts_with(sleep) {
        println!("Zzzzz");
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 400 NOT FOUND\r\n\r\n", "404.html")
    };
    let file_contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}",status_line,file_contents);

    stream.write(response.as_bytes()).unwrap();

    println!("SENT A RESPONSE");

    stream.flush().unwrap();
}