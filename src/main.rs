extern crate clap;
use clap::{Arg, Command};
use jerry::ThreadPool;
use log::{error, info, warn};
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;

fn main() {
    env_logger::init();

    let matches = Command::new("Jerry server")
        .version("0.1.0")
        .about("Maxim Zhdanov - https://github.com/Flict-dev")
        .about("Starts the multithreaded web server")
        .arg(
            Arg::new("addr")
                .short('a')
                .long("address")
                .takes_value(true)
                .help("Socket address"),
        )
        .arg(
            Arg::new("size")
                .short('w')
                .long("workers")
                .takes_value(true)
                .help("Size of workers"),
        )
        .get_matches();

    let mut addr = "127.0.0.1:7878";
    if let Some(c_addr) = matches.value_of("addr") {
        addr = c_addr;
        info!("Now server using - {} address", addr);
    } else {
        warn!("The server uses - 127.0.0.1:7878 by default");
    }

    let mut size: usize = 4;
    if let Some(c_size) = matches.value_of("size") {
        size = c_size.parse::<usize>().unwrap();
        info!("Now server using - {} workers", size);
    } else {
        warn!("The server uses 4 workers by default");
    }

    let listener = TcpListener::bind(addr).unwrap();

    let pool = ThreadPool::new(size).unwrap_or_else(|err: &str| {
        error!("Problem with creating workers - {}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    info!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK", "templates/200.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "templates/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
