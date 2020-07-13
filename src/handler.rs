use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

const HEADER_SIZE: usize = 100;

pub enum RunType {
    Host,
    Client,
}

pub fn host_or_client<'a>() -> Result<RunType, &'a str> {
    println!("1 - Client\n2 - Host");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let choice = match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Only enter integer responses"),
    };

    match choice {
        1 => Ok(RunType::Client),
        2 => Ok(RunType::Host),
        _ => Err("You must be either a host or client to transfer files"),
    }
}

pub fn send_file(mut stream: TcpStream, path: &str) -> io::Result<()> {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if file_name.len() > HEADER_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File name too long.",
        ));
    }
    println!("File name: {}", file_name);

    let mut file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    println!("File size: {}", file_size);

    let mut buffer = read_name_into_buffer(file_name);

    let mut content_buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut content_buffer)?;
    println!("Bytes read from file: {}", read_amt);

    buffer.extend(content_buffer);

    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {}", written_amt);
    Ok(())
}

fn read_name_into_buffer(name: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; HEADER_SIZE];
    let name = name.as_bytes();
    for i in 0..name.len() {
        buffer[i] = name[i];
    }
    buffer
}
