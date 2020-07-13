use std::io;

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
