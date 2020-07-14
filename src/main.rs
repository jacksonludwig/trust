mod client;
mod handler;
mod host;

use handler::RunType;

const NUM_THREADS: usize = 4;
const SERVER_IP: &str = "0.0.0.0:7878";

fn main() -> std::io::Result<()> {
    match handler::host_or_client().unwrap() {
        RunType::Host => host::start_hosting(NUM_THREADS, SERVER_IP)?,
        RunType::Client => client::start_sending(SERVER_IP)?,
    }

    Ok(())
}
