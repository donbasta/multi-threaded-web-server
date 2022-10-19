use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use mtws::threadpool::types::ThreadPool;

const ADDRESS: &str = "127.0.0.1:7878";
const THREADPOOL_SIZE: usize = 4;
const MAX_REQUESTS: usize = 10;

fn main() {
    let (listener, pool) = prepare_server();

    for stream in listener.incoming().take(MAX_REQUESTS) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down.");
}

fn prepare_server() -> (TcpListener, ThreadPool) {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    let pool = ThreadPool::build(THREADPOOL_SIZE).unwrap_or_else(|err| {
        println!("{}", err.description);
        process::exit(1)
    });

    (listener, pool)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
