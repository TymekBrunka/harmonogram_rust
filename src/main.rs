use iced::widget::{button, column, container, text};
use iced::{Center, Element, Size, Subscription, window};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Default)]
struct App {
    value: i64,
}

static mut WINDOW_SIZE : Size = Size { width: 0.0, height: 0.0 };

fn get_window_size() -> Size {
    unsafe {return WINDOW_SIZE; }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    No,
    Resized
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                button("Increment").on_press(Message::No),
                text(self.value).size(50),
                button("Decrement").on_press(Message::No)
            ]
            .padding(20)
            .align_x(Center),
        )
            .width(get_window_size().width)
            .height(get_window_size().height)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().filter_map(|event| {
            unsafe { WINDOW_SIZE = event.1; }
            return Some(Message::Resized);
        })
    }
}
