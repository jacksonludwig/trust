use iced::{text_input, Element, Row, Sandbox, Settings, Text, TextInput};

fn main() {
    Container::run(Settings::default())
}

#[derive(Default)]
struct Container {
    ip_input: text_input::State,
    ip_input_value: String,
    menu: Menu,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
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

impl Sandbox for Container {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Trust - File Transfer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInputChanged(s) => {
                self.ip_input_value = s;
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        match self.menu {
            _ => Row::new()
                .padding(40)
                .push(TextInput::new(
                    &mut self.ip_input,
                    "IP Address",
                    &mut self.ip_input_value,
                    Message::TextInputChanged,
                ))
                .into(),
        }
    }
}
