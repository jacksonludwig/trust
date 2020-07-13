use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::result::Result;

mod host;

const NUM_THREADS: usize = 4;
const SERVER_IP: &str = "127.0.0.1:7878";

fn main() -> std::io::Result<()> {
    match host_or_client().unwrap() {
        RunType::Host => host::start_hosting(NUM_THREADS, SERVER_IP)?,
        RunType::Client => {
            let stream = TcpStream::connect(SERVER_IP)?;
            if let Err(e) = send_file(stream, "C:\\Users\\jacks\\Desktop\\test.txt") {
                panic!("The file was not able to be sent: {:?}", e);
            }
        }
    }

    Ok(())
}

enum RunType {
    Host,
    Client,
}

fn host_or_client<'a>() -> Result<RunType, &'a str> {
    println!("1 - Client\n2 - Host");
    let choice: u8;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    choice = match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Only enter integer responses"),
    };

    match choice {
        1 => Ok(RunType::Client),
        2 => Ok(RunType::Host),
        _ => Err("You must be either a host or client to transfer files"),
    }
}

fn send_file(mut stream: TcpStream, path: &str) -> io::Result<()> {
    let path = Path::new(path);
    let file_name = path.file_name().unwrap();
    println!("File name: {:?}", file_name);

    let mut file = File::open(path)?;
    let file_size = file.metadata().unwrap().len();
    println!("File size: {}", file_size);

    let mut buffer = vec![0; file_size as usize];
    let read_amt = file.read(&mut buffer)?;
    println!("Bytes read from file: {}", read_amt);

    let written_amt = stream.write(&buffer)?;
    println!("Bytes written to stream: {}", written_amt);
    Ok(())
}
