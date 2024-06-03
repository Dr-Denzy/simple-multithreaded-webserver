use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::time::Duration;

use server::ThreadPool;

fn main() {

    // _example_task();

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            // use a pool of threads to handle incoming requests
            Ok(stream) => pool.execute(|| {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Failed to handle connection: {}", e);
                }
            }),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().ok_or("Failed to read request line")??;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "webpages/hello.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "webpages/404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn _example_task() {
    let pool = ThreadPool::new(4);
    let (tx, rx) = mpsc::channel();

    for i in 0..8 {
        let tx = tx.clone();
        pool.execute(move || {
            println!("Processing task {}", i);
            tx.send(i).expect("Failed to send message");
        });
    }

    drop(tx);

    for received in rx {
        println!("Received task {}", received);
    }

    println!("All tasks completed.");
}
