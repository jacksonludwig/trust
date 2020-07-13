use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use threadpool::ThreadPool;

use std::collections::HashMap;
use std::str;

const BUFFER_SIZE: usize = 4096;

struct Host<'a, 'b> {
    ip_map: HashMap<&'a str, Vec<u8>>,
    num_workers: usize,
    listener_ip: &'b str,
}

impl<'a, 'b> Host<'a, 'b> {
    pub fn new(num_workers: usize, listener_ip: &str) -> Host {
        Host {
            ip_map: HashMap::new(),
            num_workers,
            listener_ip,
        }
    }

    pub fn start_hosting(&self) -> std::io::Result<()> {
        println!("Server started on: {}", self.listener_ip);
        create_pool(self.num_workers, self.listener_ip)?;
        Ok(())
    }

    fn create_pool(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.listener_ip)?;
        let pool = ThreadPool::new(self.num_workers);
        loop {
            let (stream, address) = listener.accept()?;
            pool.execute(move || {
                self.handle_connection(stream, address);
            });
        }
    }

    fn handle_connection(self, mut stream: TcpStream, address: SocketAddr) {
        println!("Received connection from: {}", address.to_string());
        let mut buffer = vec![0; BUFFER_SIZE];
        let bytes_read = stream.read(&mut buffer).unwrap();
        println!("bytes read: {}", bytes_read);
        println!(
            "Buffer currently holds: {:?}",
            str::from_utf8(&buffer[0..bytes_read]).unwrap()
        );
    }
}
