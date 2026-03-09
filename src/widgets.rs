use iced::Size;
// use iced::Theme;
// use std::sync::OnceLock;
// pub static DIMMED_THEME: OnceLock<Theme> = OnceLock::new();

pub static mut WINDOW_SIZE: Size = Size {
    width: 0.0,
    height: 0.0,
};

pub fn get_window_size() -> Size {
    unsafe {
        return WINDOW_SIZE;
    }
}

#[macro_export]
macro_rules! menu_button {
    ($label:expr) => {
        iced::widget::button($label).style(|_, _| iced::widget::button::Style {
            background: Some(iced::color!(0x151515).into()),
            text_color: iced::Theme::Dark.palette().text.into(),
            ..Default::default()
        })
    };
}

#[macro_export]
macro_rules! menu_bar {
    ($row:expr) => {
        iced::widget::container(
            $row
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::color!(0x151515).into()),
            ..Default::default()
        })
        .height(32)
        .width(get_window_size().width)// .center_x(Fill),
    };
}
