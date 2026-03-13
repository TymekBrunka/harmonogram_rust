use std::collections::HashMap;
use std::fs;
use std::rc::Weak;

use iced::futures::future::WeakShared;
use iced::widget::{Column, button, container, keyed_column, row, space};
use iced::{Element, Subscription, Theme, window};
use json;
use rfd::FileDialog;

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
        .theme(Theme::Dark)
        .run()
}

struct TodoItem {
    zadanie: String,
    wykonano: bool,
    notki: String,
}

struct Wpis {
    zmiana1: Vec<TodoItem>,
    zmiana2: Vec<TodoItem>,
    awarie: String,
}

#[derive(Default)]
struct App<'a> {
    entries: HashMap<String, Wpis>,
    selected_index: Option<&'a mut Wpis>
}

#[derive(Debug, Clone)]
enum Message {
    No,
    Load,
    Resized,
    Selected(String)
}

impl<'a> App<'_> {
    fn update(&mut self, message: Message) {
        match message {
            Message::Load => {
                let file = FileDialog::new()
                    .add_filter("json (harmonogram)", &["harm.json"])
                    .add_filter("json", &["harm.json", "json"])
                    .add_filter("wszystkie pliki", &["*"])
                    .set_directory("/")
                    .pick_file();
                if let Some(file) = file {
                    let parsed = json::parse(&fs::read_to_string(file).unwrap()[..]);
                    // println!("{:#?}", parsed);
                    if parsed.is_err() {
                        return;
                    }

                    let mut entries = HashMap::<String, Wpis>::new();
                    let parsed = parsed.unwrap();
                    for (key, value) in parsed.entries() {
                        // println!("{:#?}", i);
                        let mut entry = Wpis {
                            zmiana1: vec![],
                            zmiana2: vec![],
                            awarie: "".to_owned(),
                        };

                        if value.has_key("zmiana1") {
                            if let json::JsonValue::Array(zmiana1) = &value["zmiana1"] {
                                for v in zmiana1 {
                                    let mut item = TodoItem {
                                        zadanie: "".to_owned(),
                                        wykonano: false,
                                        notki: "".to_owned(),
                                    };
                                    if v.len() >= 3 {
                                        if v[0].is_string() {
                                            item.zadanie = String::from(v[0].as_str().unwrap());
                                        }
                                        if v[1].is_boolean() {
                                            item.wykonano = v[1].as_bool().unwrap();
                                        }
                                        if v[2].is_string() {
                                            item.notki = String::from(v[2].as_str().unwrap());
                                        }
                                    }
                                    entry.zmiana1.push(item);
                                }
                            }
                        }

                        if value.has_key("zmiana2") {
                            if let json::JsonValue::Array(zmiana1) = &value["zmiana1"] {
                                for v in zmiana1 {
                                    let mut item = TodoItem {
                                        zadanie: "".to_owned(),
                                        wykonano: false,
                                        notki: "".to_owned(),
                                    };
                                    if v.len() >= 3 {
                                        if v[0].is_string() {
                                            item.zadanie = String::from(v[0].as_str().unwrap());
                                        }
                                        if v[1].is_boolean() {
                                            item.wykonano = v[1].as_bool().unwrap();
                                        }
                                        if v[2].is_string() {
                                            item.notki = String::from(v[2].as_str().unwrap());
                                        }
                                    }
                                    entry.zmiana2.push(item);
                                }
                            }
                        }

                        if value.has_key("awarie") {
                            if let Some(val) = value["awarie"].as_str() {
                                entry.awarie = String::from(val);
                            }
                        }
                        entries.insert(key.to_string(), entry);
                    }
                    self.entries = entries;
                }
            },
            Message::Selected(sidx) => {
                self.selected_index = Some(self.entries.get_mut(&sidx).unwrap());
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
            (
                1,
                // colored_box!(row![], color!(0xadb34))
                row![
                    container(
                        container({
                            let mut items = Vec::<Element<'_, _, _, _>>::new();
                            for (k, _) in &self.entries {
                                let item =
                                    button(k.as_str()).on_press(Message::Selected(k.clone())).width(140).into();
                                items.push(item);
                            }
                            Column::with_children(items)
                                .width(150)
                                .clip(true)
                                .spacing(5)
                                .padding(5)
                        })
                        .height(get_window_size().height - 57.0)
                        .style(container::bordered_box)
                    )
                    .padding(10),
                    container(
                        container("hello")
                            .style(container::bordered_box)
                            .width(100)
                            .height(100)
                    )
                    .width(get_window_size().width - 180.0)
                    .center_x(get_window_size().width - (170.0 * 2.0) - 5.0)
                    .center_y(get_window_size().height - 52.0)
                ]
            )
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
