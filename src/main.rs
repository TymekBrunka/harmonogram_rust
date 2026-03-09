use iced::widget::{button, container, row, space};
use iced::{Element, Subscription, Theme, color, window};

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
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    No,
    Resized,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            menu_bar!(
                row![
                    space().width(10),
                    menu_button!("Załaduj").on_press(Message::No),
                    menu_button!("Zapisz").on_press(Message::No),
                ]
            )
        )
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
