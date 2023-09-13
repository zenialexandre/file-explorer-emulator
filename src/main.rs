mod window_helper;
mod rename_operation;
mod delete_operation;
mod create_operation;
mod copy_and_paste_operation;
mod conflict_process;

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus::html::input_data::keyboard_types::{Code, Modifiers};
use std::sync::{Mutex};
use chrono::{DateTime, Utc};

#[macro_use]
extern crate lazy_static;

lazy_static! { static ref CLICKED_DIRECTORY_ID: Mutex<usize> = Mutex::new(0); }
lazy_static! { static ref NEW_FILE_OR_DIR_NAME: Mutex<String> = Mutex::new("".to_string()); }

const REGULAR_FILE: &str = "Regular File";

#[derive(Clone)]
pub(crate) struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    error: Option<String>,
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_disable_context_menu(true).with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1000.0, 800.0))
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_theme(Option::from(dioxus_desktop::tao::window::Theme::Dark))
            .with_focused(true)
        )
    );
}

fn app(cx: Scope) -> Element {
    let main_element: &UseRef<Vec<Event<MountedData>>> = use_ref(cx, || Vec::new());
    let files: &UseRef<Files> = use_ref(cx, Files::new);

    cx.render(rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            style { include_str!("./assets/styles.css") }
            header {
                i { class: "material-icons", onclick: move |_| files.write().walk_to_last_directory(), "arrow_back" }
                i { class: "material-icons", onclick: move |_| window_helper::validate_clicked_id_on_click(files, &CLICKED_DIRECTORY_ID), "arrow_forward" }
                h1 { files.read().current() }
                span { }
                i { class: "material-icons", onclick: move |_| dioxus_desktop::use_window(cx).close(), "cancel" }
            },
            div {
                tabindex: "0",
                onmounted: move |element| { main_element.write().push(element); },
                onclick: move |_| {
                    if let Some(element) = main_element.read().last() {
                        element.set_focus(true);
                    }
                },
                onkeydown: move |keydown_event| {
                    if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyN {
                        let create_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup, create_rename_popupProps { files_props: files.clone(), title_props: "Create" });
                        window_helper::create_new_dom_generic_window(cx, create_dom, "Create");
                    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyV {
                        copy_and_paste_operation::execute_paste_operation(cx, files);
                    }
                },
                main {
                    files.read().path_names.iter().enumerate().map(|(directory_id, path)| {
                        let path_end = path.split('\\').last().unwrap_or(path.as_str());
                        let icon_type: String = window_helper::get_icon_type(path.to_string());
                        let file_type: String = window_helper::get_file_type_formatted(path.to_string());
                        let path_metadata = std::fs::metadata(path.to_string());
                        let file_size: u64 = window_helper::get_file_size(path.to_string());
                        let mut last_modification_date_utc: DateTime<Utc> = Default::default();
                        let mut _last_modification_date_formatted: String = String::new();

                        if let Ok(path_metadata) = path_metadata.expect("Modified").modified() {
                            last_modification_date_utc = path_metadata.into();
                        }
                        _last_modification_date_formatted = last_modification_date_utc.format("%d/%m/%Y %H:%M:%S").to_string().split('.').next().expect("Next").to_string();

                        rsx! (
                            div {
                                class: "folder",
                                key: "{path}",
                                table {
                                    class: "explorer-table",
                                    tbody {
                                        class: "explorer-tbody",
                                        tr {
                                            tabindex: "0",
                                            onkeydown: move |keydown_event| {
                                                if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyR {
                                                    let rename_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup, create_rename_popupProps { files_props: files.clone(), title_props: "Rename" });
                                                    window_helper::create_new_dom_generic_window(cx, rename_dom, "Rename");
                                                } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyD {
                                                    let delete_dom: VirtualDom = VirtualDom::new_with_props(delete_popup, delete_popupProps { files_props: files.clone() });
                                                    window_helper::create_new_dom_generic_window(cx, delete_dom, "Delete");
                                                } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyC {
                                                    copy_and_paste_operation::execute_copy_operation(&CLICKED_DIRECTORY_ID, files);
                                                }
                                            },
                                            ondblclick: move |_| {
                                                let selected_full_path = window_helper::get_selected_full_path(files, &CLICKED_DIRECTORY_ID);
                                                match std::fs::metadata(selected_full_path.clone()) {
                                                    Ok(path_metadata) => {
                                                        if path_metadata.is_file() {
                                                            window_helper::open_file(selected_full_path.clone().as_str());
                                                        } else if path_metadata.is_dir() {
                                                            files.write().enter_directory(directory_id);
                                                        }
                                                    },
                                                    Err(error) => panic!("{}", error)
                                                }
                                            },
                                            onclick: move |_| { *CLICKED_DIRECTORY_ID.lock().unwrap() = directory_id; },
                                            td { i { class: "material-icons", "{icon_type}" } },
                                            td { class: "explorer-tbody-td", h1 { "{path_end}" } },
                                            td { class: "explorer-tbody-td", h1 { "{_last_modification_date_formatted}" } },
                                            td { class: "explorer-tbody-td", h1 { "{file_type}" } },
                                            if window_helper::get_file_type_formatted(path.to_string()) == REGULAR_FILE.to_string() {
                                                rsx!( td { class: "explorer-tbody-td", h1 { "{file_size} KB" } } )
                                            } else {
                                                rsx!( td { class: "explorer-tbody-td", h1 { " " } } )
                                            }
                                        }
                                    }
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
        }
    })
}

#[inline_props]
fn create_rename_popup<'a>(cx: Scope, files_props: UseRef<Files>, title_props: &'a str) -> Element {
    cx.render(rsx! {
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", },
            style { include_str!("./assets/create_rename_popup.css") }
            div {
                class: "central-div",
                h1 { "Enter new directory/file name: " },
                div {
                    class: "forms-div",
                    input {
                        oninput: |input_event| { *NEW_FILE_OR_DIR_NAME.lock().unwrap() = input_event.value.to_string() },
                        r#type: "text",
                        placeholder: "Directory/File new name"
                    },
                    br {}
                    i {
                        class: "material-icons",
                        onclick: move |_| {
                           dioxus_desktop::use_window(cx).close();
                        },
                        "cancel"
                    },
                    i {
                        class: "material-icons",
                        onclick: move |_| {
                            if *NEW_FILE_OR_DIR_NAME.lock().unwrap().trim() != "".to_string() {
                                match title_props.as_ref() {
                                    "Rename" => rename_operation::execute_rename_operation(files_props, &CLICKED_DIRECTORY_ID, &NEW_FILE_OR_DIR_NAME),
                                    "Create" => create_operation::execute_create_operation(files_props, &NEW_FILE_OR_DIR_NAME),
                                    _ => println!("Something gone wrong.")
                                }
                                dioxus_desktop::use_window(cx).close();
                            }
                        },
                        "check_circle"
                    }
                }
            }
        }
    })
}

#[inline_props]
fn delete_popup(cx: Scope, files_props: UseRef<Files>) -> Element {
    cx.render(rsx! {
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", },
            style { include_str!("./assets/delete_popup.css") },
            div {
                class: "central-div",
                i { class: "material-icons", {}, "warning" }
                h1 { "Do you really wish to delete this file/directory? " }
                br {}
                i {
                    class: "material-icons",
                    onclick: move |_| {
                        dioxus_desktop::use_window(cx).close();
                    },
                    "cancel"
                },
                i {
                    class: "material-icons",
                    onclick: move |_| {
                        delete_operation::execute_delete_operation(files_props, &CLICKED_DIRECTORY_ID);
                        dioxus_desktop::use_window(cx).close();
                    },
                    "check_circle"
                }
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
        window_helper::clean_lazy_static_value(&CLICKED_DIRECTORY_ID);
        self.reload_path_list();
    }

    fn enter_directory(&mut self, directory_id: usize) {
        let path = &self.path_names[directory_id];
        self.path_stack.push(path.clone());
        window_helper::clean_lazy_static_value(&CLICKED_DIRECTORY_ID);
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        return self.path_stack.last().unwrap();
    }

    fn clear_error(&mut self) {
        self.error = None;
    }

}
