use nfd2::Response;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use std::fs::File;
use std::net::TcpStream;

const HEADER_SIZE: usize = 100;

pub fn start_sending(server_ip: &str) -> std::io::Result<(String, usize, usize)> {
    let stream = TcpStream::connect(server_ip)?;
    let file = find_file()?;
    let (name, size, written) = send_file(stream, &file)?;

    Ok((name, size, written))
}

fn find_file() -> std::io::Result<String> {
    let file = match nfd2::open_file_dialog(None, None).unwrap() {
        Response::Okay(file_path) => file_path,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error selecting file",
            ))
        }
    };
    Ok(file.to_str().unwrap().to_string())
}

fn send_file(mut stream: TcpStream, path: &str) -> io::Result<(String, usize, usize)> {
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
    let file_size = file.metadata().unwrap().len() as usize;
    println!("File size: {}", file_size);

    let mut buffer = read_name_into_buffer(file_name);

    let mut content_buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut content_buffer)?;
    println!("Bytes read from file: {}", read_amt);

    buffer.extend(content_buffer);

    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {}", written_amt);
    Ok((file_name.to_string(), file_size, written_amt))
}

fn read_name_into_buffer(name: &str) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; HEADER_SIZE];
    let name = name.as_bytes();
    for i in 0..name.len() {
        buffer[i] = name[i];
    }
    buffer
}
