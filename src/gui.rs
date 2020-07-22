use iced::{
    button, text_input, Button, Column, Container, Element, HorizontalAlignment, Length, Row,
    Sandbox, Settings, Text, TextInput,
};

use super::{host, client};

const NUM_WORKERS: usize = 6;

pub fn start() {
    Trust::run(Settings::default())
}

#[derive(Default)]
struct Trust {
    ip_input: text_input::State,
    ip_input_value: String,
    host_button: button::State,
    connect_button: button::State,
    status_value: String,

    theme: style::Theme,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    HostButtonPressed,
    ConnectButtonPressed,
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
            //TODO: Replace placeholder errors with visible errors
            //TODO: Multithread
            //TODO: Change base path
            Message::HostButtonPressed => {
                match host::start_hosting(NUM_WORKERS, &self.ip_input_value) {
                    Err(e) => println!("placeholder error: {}", e),
                    Ok(()) => println!("placeholder data"),
                }
                self.status_value = String::from("Hosting");
            }
            Message::ConnectButtonPressed => {
                match client::start_sending(&self.ip_input_value) {
                    Err(e) => println!("placeholder error: {}", e),
                    Ok((name, size, written)) => println!("placeholder data: {}, {}, {}", name, size, written)
                }
                self.status_value = String::from("Connected");
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
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
            .size(30)
            .style(self.theme);

            let host_btn = Button::new(
                &mut self.host_button,
                Text::new("Host").horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::HostButtonPressed)
            .width(Length::Units(100))
            .style(self.theme);

            let connect_btn = Button::new(
                &mut self.connect_button,
                Text::new("Connect").horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::ConnectButtonPressed)
            .width(Length::Units(100))
            .style(self.theme);

            let button_row = Row::new().spacing(20).push(host_btn).push(connect_btn);
            let buttons = Container::new(button_row)
                .width(Length::Fill)
                .center_x()
                .center_y();

            let status_text = Text::new(&self.status_value)
                .width(Length::Fill)
                .size(35)
                .color([0.5, 0.5, 0.5])
                .horizontal_alignment(HorizontalAlignment::Center);

            let main_column = Column::new()
                .max_width(800)
                .spacing(20)
                .push(title)
                .push(input)
                .push(buttons)
                .push(status_text);

            Container::new(main_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(self.theme)
                .into()
    }
}

mod style {
    use iced::{button, container, text_input};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Dark,
    }

    impl Theme {
        pub const ALL: [Theme; 1] = [Theme::Dark];
    }

    impl Default for Theme {
        fn default() -> Self {
            Theme::Dark
        }
    }

    impl From<Theme> for Box<dyn container::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Dark => dark::Container.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn text_input::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Dark => dark::TextInput.into(),
            }
        }
    }

    impl From<Theme> for Box<dyn button::StyleSheet> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Dark => dark::Button.into(),
            }
        }
    }

    mod dark {
        use iced::{button, container, text_input, Background, Color};

        const SURFACE: Color = Color::from_rgb(
            0x40 as f32 / 255.0,
            0x44 as f32 / 255.0,
            0x4B as f32 / 255.0,
        );

        const ACCENT: Color = Color::from_rgb(
            0x6F as f32 / 255.0,
            0xFF as f32 / 255.0,
            0xE9 as f32 / 255.0,
        );

        const ACTIVE: Color = Color::from_rgb(
            0x72 as f32 / 255.0,
            0x89 as f32 / 255.0,
            0xDA as f32 / 255.0,
        );

        const HOVERED: Color = Color::from_rgb(
            0x67 as f32 / 255.0,
            0x7B as f32 / 255.0,
            0xC4 as f32 / 255.0,
        );

        pub struct Container;

        impl container::StyleSheet for Container {
            fn style(&self) -> container::Style {
                container::Style {
                    background: Some(Background::Color(Color::from_rgb8(0x36, 0x39, 0x3F))),
                    text_color: Some(Color::WHITE),
                    ..container::Style::default()
                }
            }
        }

        pub struct TextInput;

        impl text_input::StyleSheet for TextInput {
            fn active(&self) -> text_input::Style {
                text_input::Style {
                    background: Background::Color(SURFACE),
                    border_radius: 2,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            }

            fn focused(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1,
                    border_color: ACCENT,
                    ..self.active()
                }
            }

            fn hovered(&self) -> text_input::Style {
                text_input::Style {
                    border_width: 1,
                    border_color: Color { a: 0.3, ..ACCENT },
                    ..self.focused()
                }
            }

            fn placeholder_color(&self) -> Color {
                Color::from_rgb(0.4, 0.4, 0.4)
            }

            fn value_color(&self) -> Color {
                Color::WHITE
            }

            fn selection_color(&self) -> Color {
                ACTIVE
            }
        }

        pub struct Button;

        impl button::StyleSheet for Button {
            fn active(&self) -> button::Style {
                button::Style {
                    background: Some(Background::Color(ACTIVE)),
                    border_radius: 3,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }

            fn hovered(&self) -> button::Style {
                button::Style {
                    background: Some(Background::Color(HOVERED)),
                    text_color: Color::WHITE,
                    ..self.active()
                }
            }

            fn pressed(&self) -> button::Style {
                button::Style {
                    border_width: 1,
                    border_color: Color::WHITE,
                    ..self.hovered()
                }
            }
        }
    }
}
