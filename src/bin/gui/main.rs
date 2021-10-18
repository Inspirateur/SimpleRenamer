#![windows_subsystem = "windows"]
mod style;
use crate::style::*;
use iced::{
    button, executor, scrollable, text_input, window, Application, Button, Column, Command,
    Container, Element, HorizontalAlignment, Length, Row, Scrollable, Settings, Text, TextInput,
};
use itertools::Itertools;
use nfd2::Response;
use pathdiff::diff_paths;
use srenamer::{apply_rename, get_rule_rep, rename_map};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::{env, path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
enum Message {
    EditFileName(String),
    Apply,
    Browse,
}

#[derive(Default)]
struct Flags {
    cwd: PathBuf,
    file: String,
}

#[derive(Default)]
struct Srenamer {
    rename_field: text_input::State,
    input_value: String,
    apply: button::State,
    browse: button::State,
    preview: scrollable::State,
    file: String,
    cwd: PathBuf,
    rename: HashMap<PathBuf, PathBuf>,
    palette: Palette,
    error_msg: String,
    should_exit: bool,
}

impl Application for Srenamer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let file = get_rule_rep(&flags.cwd, &flags.file);
        (
            Self {
                file: file.clone(),
                cwd: flags.cwd.clone(),
                input_value: file.clone(),
                rename: rename_map(&flags.cwd, &file, &PathBuf::from(file.clone())),
                should_exit: false,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!(
            "Simple renamer - {}/",
            self.cwd.file_name().unwrap().to_string_lossy()
        )
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        self.error_msg = String::new();
        match message {
            Message::EditFileName(value) => {
                self.input_value = value;
                self.rename = rename_map(
                    &self.cwd,
                    &self.file,
                    &PathBuf::from_str(&self.input_value).unwrap(),
                );
            }
            Message::Browse => match nfd2::open_pick_folder(None) {
                Ok(response) => match response {
                    Response::Okay(folder_path) => {
                        if let Some(diff) = diff_paths(&folder_path, &self.cwd) {
                            self.input_value = diff
                                .join(
                                    PathBuf::from_str(&self.input_value)
                                        .unwrap()
                                        .file_name()
                                        .unwrap_or(OsStr::new(""))
                                        .to_string_lossy()
                                        .to_string(),
                                )
                                .to_string_lossy()
                                .to_string()
                                .replace("\\", "/");
                            self.rename = rename_map(
                                &self.cwd,
                                &self.file,
                                &PathBuf::from_str(&self.input_value).unwrap(),
                            );
                        } else {
                            self.error_msg =
                                format!("Couldn't get relative path from {:?}", folder_path);
                        }
                    }
                    Response::OkayMultiple(_folder_path) => {
                        self.error_msg = String::from("Cannot select multiple folders.")
                    }
                    Response::Cancel => {}
                },
                Err(err) => {
                    self.error_msg = format!("Error: {}", err);
                }
            },
            Message::Apply => match apply_rename(&self.rename) {
                Ok(()) => self.should_exit = true,
                Err(err) => self.error_msg = format!("{}", err),
            },
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut preview = Scrollable::new(&mut self.preview).style(self.palette);
        for (i, (old, new)) in self
            .rename
            .iter()
            .sorted_by_key(|(old, _new)| *old)
            .enumerate()
        {
            let mut container = Container::new(
                Column::new()
                    .padding(5)
                    .spacing(5)
                    .push(
                        Text::new(old.file_name().unwrap().to_string_lossy())
                            .size(16)
                            .color(self.palette.greyed),
                    )
                    .push(
                        Text::new(new.file_name().unwrap().to_string_lossy())
                            .size(16)
                            .color(self.palette.font),
                    ),
            )
            .width(iced::Length::Fill);
            if i % 2 == 0 {
                container = container.style(RowEven(self.palette))
            } else {
                container = container.style(RowOdd(self.palette))
            }
            preview = preview.push(Row::new().push(container));
        }
        Container::new(
            Column::new()
                .spacing(20)
                .padding(20)
                .push(
                    Column::new()
                        .spacing(5)
                        .push(Text::new(&self.file).size(16).color(self.palette.greyed))
                        .push(
                            Row::new()
                                .spacing(20)
                                .push(
                                    TextInput::new(
                                        &mut self.rename_field,
                                        "new file name",
                                        &self.input_value,
                                        Message::EditFileName,
                                    )
                                    .padding(5)
                                    .size(16)
                                    .style(self.palette),
                                )
                                .push(
                                    Button::new(
                                        &mut self.browse,
                                        Text::new("Change Dest.")
                                            .size(16)
                                            .horizontal_alignment(HorizontalAlignment::Center),
                                    )
                                    .padding(5)
                                    .on_press(Message::Browse)
                                    .style(self.palette),
                                ),
                        )
                        .push(
                            Text::new(&self.error_msg)
                                .size(14)
                                .color(self.palette.error),
                        ),
                )
                .push(preview.height(Length::Fill))
                .push(
                    Container::new(
                        Button::new(&mut self.apply, Text::new("Apply").size(20))
                            .on_press(Message::Apply)
                            .style(ApplyButton(self.palette)),
                    )
                    .width(Length::Fill)
                    .center_x(),
                ),
        )
        .style(self.palette)
        .into()
    }
}

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    let mut cwd = env::current_dir().unwrap();
    let file = if args.len() == 1 {
        match nfd2::open_file_dialog(None, None) {
            Ok(response) => match response {
                Response::Okay(file_path) => {
                    cwd = file_path.parent().unwrap().to_path_buf();
                    file_path.file_name().unwrap().to_string_lossy().to_string()
                }
                Response::OkayMultiple(file_paths) => {
                    let file_path = &file_paths[0];
                    cwd = file_path.parent().unwrap().to_path_buf();
                    file_path.file_name().unwrap().to_string_lossy().to_string()
                }
                Response::Cancel => return Ok(()),
            },
            Err(err) => {
                println!("{:?}", err);
                return Ok(());
            }
        }
    } else {
        let file_path = PathBuf::from(args[1].clone());
        if let Some(parent) = file_path.parent() {
            cwd.push(parent);
        }
        file_path.file_name().unwrap().to_string_lossy().to_string()
    };
    Srenamer::run(Settings {
        window: window::Settings {
            size: (800, 500),
            ..Default::default()
        },
        flags: Flags {
            file: file,
            cwd: cwd,
        },
        ..Default::default()
    })
}
