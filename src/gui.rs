use iced::{button, Align, Button, Element, Row, Sandbox, Text};

use super::client;
use super::host;

const NUM_THREADS: usize = 4;
const SERVER_IP: &str = "192.168.1.217:7878";

#[derive(Default)]
pub struct App {
    host_button: button::State,
    client_button: button::State,
    send_file_button: button::State,
    start_server_button: button::State,
    menu: Menu,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    HostPressed,
    ClientPressed,
    SendPressed,
    StartPressed,
}

pub enum Menu {
    Main,
    Host,
    Client,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::Main
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Test - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HostPressed => {
                self.menu = Menu::Host;
            }
            Message::ClientPressed => {
                self.menu = Menu::Client;
            }
            // TODO: stop these from blocking UI and handle errors
            Message::SendPressed => client::start_sending(SERVER_IP).unwrap(),
            Message::StartPressed => host::start_hosting(NUM_THREADS, SERVER_IP).unwrap(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.menu {
            Menu::Client => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("client menu placeholder".to_string()).size(50))
                .push(
                    Button::new(&mut self.send_file_button, Text::new("Send File"))
                        .on_press(Message::SendPressed),
                )
                .into(),
            Menu::Host => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("host menu placeholder".to_string()).size(50))
                .push(
                    Button::new(&mut self.start_server_button, Text::new("Start Server"))
                        .on_press(Message::StartPressed),
                )
                .into(),
            Menu::Main => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.host_button, Text::new("Host"))
                        .on_press(Message::HostPressed),
                )
                .push(
                    Button::new(&mut self.client_button, Text::new("Client"))
                        .on_press(Message::ClientPressed),
                )
                .into(),
        }
    }
}
