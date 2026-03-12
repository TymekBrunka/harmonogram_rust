use std::collections::HashMap;
use std::fs;

use iced::widget::{Container, Row, button, container, keyed_column, row, space};
use iced::{Element, Subscription, Theme, color, window};
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
struct App {
    entries: HashMap<String, Wpis>,
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
                    .add_filter("wszystkie pliki", &["*"])
                    .set_directory("/")
                    .pick_file();
                let parsed = json::parse(&fs::read_to_string(file.unwrap()).unwrap()[..]);
                // println!("{:#?}", parsed);
                if parsed.is_err() {
                    println!("e e ");
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
                container(
                    container("")
                        .style(container::bordered_box)
                        .width(40)
                        .height(40)
                )
                .padding(10)
            ) // (1, {
              //     let mut items = Vec::<Element<'_, _, _, _>>::new();
              //     for (k, _) in &self.entries {
              //         items.push(button(k.as_str()).into());
              //         println!("{k}");
              //     }
              //     Row::with_children(items)
              // }),
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
