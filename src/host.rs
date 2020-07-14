use std::fs::File;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

use threadpool::ThreadPool;

use std::str;

const HEADER_SIZE: usize = 100;
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

    let mut buffer = vec![0; HEADER_SIZE + BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes read: {}", bytes_read);

    let file_name = str::from_utf8(&buffer[0..HEADER_SIZE]).unwrap();
    println!("File name: {}", file_name);

    let file_contents = &buffer[HEADER_SIZE..bytes_read];

    save_file(file_name, file_contents.to_vec()).unwrap();
}

fn save_file(name: &str, contents: Vec<u8>) -> std::io::Result<()> {
    let base_path = "C:/Users/jacks/Desktop/";
    let name = name.to_string();

    let path_name = name.trim_end_matches(char::from(0)).to_string();
    let path = [base_path, &path_name].join("");

    let mut file = File::create(path.clone())?;
    file.write_all(&contents)?;
    println!("File saved to: {}", path);

    Ok(())
}
