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
    HostButtonPressed,
    ConnectButtonPressed,
}

enum Menu {
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
            //TODO: Start hosting server
            Message::HostButtonPressed => {}
            //TODO: Send a file
            Message::ConnectButtonPressed => {}
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        match self.menu {
            _ => {
                let title = Text::new("Trust")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);

                let input = TextInput::new(
                    &mut self.ip_input,
                    "IP Address",
                    &self.ip_input_value,
                    Message::TextInputChanged,
                )
                .padding(15)
                .size(30);

                let host_btn = Button::new(
                    &mut self.host_button,
                    Text::new("Host").horizontal_alignment(HorizontalAlignment::Center),
                )
                .width(Length::Units(100));

                let connect_btn = Button::new(
                    &mut self.connect_button,
                    Text::new("Connect").horizontal_alignment(HorizontalAlignment::Center),
                )
                .width(Length::Units(100));

                let button_row = Row::new().spacing(20).push(host_btn).push(connect_btn);
                let buttons = Container::new(button_row)
                    .width(Length::Fill)
                    .center_x()
                    .center_y();

                let main_column = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(input)
                    .push(buttons);

                Container::new(main_column)
                    .width(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
            }
        }
    }
}
