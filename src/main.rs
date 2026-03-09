use std::fs;

use iced::widget::{button, container, keyed_column, row, space};
use iced::{Element, Subscription, Theme, color, window};
use rfd::FileDialog;
use json;

mod widgets;
use widgets::*;

pub fn main() -> iced::Result {
    // DIMMED_THEME.get_or_init(|| {
    //     // let mut p: Palette = Theme::Dark.palette();
    //     // return Theme::custom("dimmed", p);
    //     Theme::Dark
    // });
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .theme(Theme::Nord)
        .run()
}

#[derive(Default)]
struct App {
}

#[derive(Debug, Clone, Copy)]
enum Message {
    No,
    Load,
    Resized,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Load => {
                let file = FileDialog::new()
                    .add_filter("json (harmonogram)", &["harm.json"])
                    .add_filter("json", &["harm.json", "json"])
                    .set_directory("/")
                    .pick_file();
                let parsed = json::parse(&fs::read_to_string(file.unwrap()).unwrap()[..]);
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(keyed_column![
            (
                0,
                menu_bar!(row![
                    space().width(10),
                    menu_button!("Załaduj").on_press(Message::Load),
                    menu_button!("Zapisz").on_press(Message::No),
                ])
            ),
        ])
        .width(get_window_size().width)
        .height(get_window_size().height)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().filter_map(|event| {
            unsafe {
                WINDOW_SIZE = event.1;
            }
            return Some(Message::Resized);
        })
    }
}
