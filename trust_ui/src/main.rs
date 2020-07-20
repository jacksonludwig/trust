use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Sandbox, Settings, Text, TextInput,
};

fn main() {
    Trust::run(Settings::default())
}

#[derive(Default)]
struct Trust {
    ip_input: text_input::State,
    ip_input_value: String,
    host_button: button::State,
    connect_button: button::State,
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

impl Sandbox for Trust {
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
            _ => Container::new(self.generate_main_column())
                .width(Length::Fill)
                .center_x()
                .into(),
        }
    }
}

// These are helper functions for creating UI elements.
impl Trust {
    fn generate_main_column(&mut self) -> Column<Message> {
        Column::new()
            .max_width(800)
            .spacing(20)
            .push(self.generate_main_title())
            .push(self.generate_ip_input())
            .push(self.generate_button_row())
    }

    fn generate_main_title(&self) -> Text {
        Text::new("Trust")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center)
    }

    fn generate_ip_input(&mut self) -> TextInput<Message> {
        TextInput::new(
            &mut self.ip_input,
            "IP Address",
            &self.ip_input_value,
            Message::TextInputChanged,
        )
        .padding(15)
        .size(30)
    }

    fn generate_button_row(&mut self) -> Row<Message> {
        Row::new()
            .padding(20)
            .push(Button::new(&mut self.host_button, Text::new("HOST")))
            .push(Button::new(&mut self.connect_button, Text::new("CONNECT")))
    }
}
