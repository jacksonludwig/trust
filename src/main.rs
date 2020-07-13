use std::net::TcpStream;

mod handler;
mod host;

use handler::RunType;

const NUM_THREADS: usize = 4;
const SERVER_IP: &str = "127.0.0.1:7878";

fn main() -> std::io::Result<()> {
    match handler::host_or_client().unwrap() {
        RunType::Host => host::start_hosting(NUM_THREADS, SERVER_IP)?,
        RunType::Client => {
            let stream = TcpStream::connect(SERVER_IP)?;
            if let Err(e) = handler::send_file(stream, "C:\\Users\\jacks\\Desktop\\test.txt") {
                panic!("The file was not able to be sent: {:?}", e);
            }
        }
    }

    Ok(())
}
