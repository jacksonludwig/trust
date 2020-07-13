use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use threadpool::ThreadPool;

use std::str;

const BUFFER_SIZE: usize = 4096;

pub fn start_hosting(num_workers: usize, listener_ip: &str) -> std::io::Result<()> {
    println!("Server started on: {}", listener_ip);
    create_pool(num_workers, listener_ip)?;
    Ok(())
}

fn create_pool(num_workers: usize, listener_ip: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(listener_ip)?;
    let pool = ThreadPool::new(num_workers);
    loop {
        let (stream, address) = listener.accept()?;
        pool.execute(move || {
            handle_connection(stream, address);
        });
    }
}

fn handle_connection(mut stream: TcpStream, address: SocketAddr) {
    println!("Received connection from: {}", address.to_string());
    let mut buffer = vec![0; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes read: {}", bytes_read);
    println!(
        "Buffer currently holds: {:?}",
        str::from_utf8(&buffer[0..bytes_read]).unwrap()
    );
}
