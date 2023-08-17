mod general_helper;
mod window_helper;

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use std::sync::Mutex;
use chrono::{DateTime, Utc};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CLICKED_DIRECTORY_ID: Mutex<usize> = Mutex::new(0);
}

pub struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    error: Option<String>,
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1000.0, 800.0))
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/simple_face_icon.png"))
            .with_theme(Option::from(dioxus_desktop::tao::window::Theme::Dark))
        )
    );
}

fn app(cx: Scope) -> Element {
    let files = use_ref(cx, Files::new);
    //let selected_record = use_state(cx, || false);
    //let selected_style = if **selected_record { "#c6c6c6" } else { "" };

    cx.render(rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            style { include_str!("./assets/styles.css") }
            header {
                i { class: "material-icons", onclick: move |_| files.write().walk_to_last_directory(), "arrow_back" }
                i { class: "material-icons", onclick: move |_| window_helper::validate_clicked_id_on_click(files, &CLICKED_DIRECTORY_ID), "arrow_forward" }
                h1 { files.read().current() }
                span { }
                i { class: "material-icons", onclick: move |_| window_helper::close_application(cx), "cancel" }
            }
            main {
                files.read().path_names.iter().enumerate().map(|(directory_id, path)| {
                    //let inner_file_div = "";
                    let path_end = path.split('/').last().unwrap_or(path.as_str());
                    let icon_type: String = window_helper::get_icon_type(path_end);
                    let last_modification_date = std::fs::metadata(path.to_string());
                    let mut last_modification_date_utc: DateTime<Utc> = Default::default();
                    let mut last_modification_date_formatted: String = String::new();

                    if let Ok(last_modification_date) = last_modification_date.expect("Modified").modified() {
                        last_modification_date_utc = last_modification_date.into();
                    }
                    last_modification_date_formatted = last_modification_date_utc.format("%d/%m/%Y %H:%M:%S").to_string().split('.').next().expect("Next").to_string();

                    rsx! (
                        div {
                            ondblclick: move |_| files.write().enter_directory(directory_id),
                            onclick: move |event| {
                                event.stop_propagation();
                                *CLICKED_DIRECTORY_ID.lock().unwrap() = directory_id;
                                //selected_record.set(true);
                            },
                            class: "folder",
                            key: "{path}",
                            //background: selected_style,
                            //"{inner_file_div}"
                            div {
                                i { class: "material-icons", "{icon_type}" },
                                h1 { class: "path-end", "{path_end}" },
                                h1 { class: "last-modification-date", "{last_modification_date_formatted}" }
                            }
                        }
                    )
                }),

                files.read().error.as_ref().map(|err| {
                    rsx! (
                        div {
                            code { "{err}" }
                            button { onclick: move |_| files.write().clear_error(), "x" }
                        }
                    )
                })
            }
        }
    })
}

impl Files {
    fn new() -> Self {
      let mut files = Self {
          path_stack: vec!["C://".to_string()],
          path_names: vec![],
          error: None,
      };
      files.reload_path_list();
      files
    }

    fn reload_path_list(&mut self) {
        let current_path = self.path_stack.last().unwrap();
        let paths = match std::fs::read_dir(current_path) {
            Ok(e) => e,
            Err(error) => {
                let error = format!("An error occurred: {error:?}");
                self.error = Some(error);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();

        // clear current state
        self.clear_error();
        self.path_names.clear();

        for path in collected {
            self.path_names.push(path.unwrap().path().display().to_string());
        }
    }

    fn walk_to_last_directory(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        window_helper::clean_clicked_directory_id(&CLICKED_DIRECTORY_ID);
        self.reload_path_list();
    }

    fn enter_directory(&mut self, directory_id: usize) {
        let path = &self.path_names[directory_id];
        self.path_stack.push(path.clone());
        window_helper::clean_clicked_directory_id(&CLICKED_DIRECTORY_ID);
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        return self.path_stack.last().unwrap();
    }

    fn clear_error(&mut self) {
        self.error = None;
    }
}
