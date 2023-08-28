use std::sync::{Mutex};
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;

use crate::{Files};
use crate::window_helper::get_converted_usize_from_string;
use crate::window_helper::load_icon_by_path;
use crate::window_helper::close_application;

lazy_static! {
    static ref FILES_VECTOR: Mutex<&'static UseRef<Files>> = Mutex::from(Files::new);
}

lazy_static! {
    static ref CLICKED_DIRECTORY_ID_VECTOR: Mutex<Vec<&'static Mutex<usize>>> = Mutex::new(vec![]);
}

lazy_static! {
    static ref NEW_FILE_OR_DIR_NAME: Mutex<String> = Mutex::new("".to_string());
}

pub fn rename_event(context: Scope, files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    *FILES_VECTOR.lock().unwrap() = files;
    *CLICKED_DIRECTORY_ID_VECTOR.lock().unwrap().push(clicked_directory_id);
    fire_rename_popup(context);
}

fn fire_rename_popup(context: Scope) {
    let window = dioxus_desktop::use_window(context);
    let dom = VirtualDom::new(rename_popup);
    window.new_window(dom, Config::default()
        .with_window(WindowBuilder::new()
            .with_resizable(false).with_focused(true)
            .with_closable(false).with_drag_and_drop(false).with_skip_taskbar(true)
            .with_window_icon(load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_title("Rename").with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(600.0, 300.0)))
    );
}

fn rename_popup(context: Scope) -> Element {
    context.render(rsx! {
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", },
            style { include_str!("./assets/rename_popup.css") }

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
                            close_application(context);
                        },
                        "cancel"
                    },
                    i {
                        class: "material-icons",
                        onclick: move |_| {
                            if *NEW_FILE_OR_DIR_NAME.lock().unwrap().trim() != "" {
                                execute_rename_operation();
                                close_application(context);
                            }
                        },
                        "check_circle"
                    }
                }
            }
        }
    })
}

fn execute_rename_operation() {
    let files: &UseRef<Files> = *FILES_VECTOR.lock().unwrap().get(0);
    let clicked_directory_id: Mutex<usize> = *CLICKED_DIRECTORY_ID_VECTOR.lock().unwrap().get(0);

    let converted_clicked_directory_id: usize = get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());
    let selected_full_path: String = files.read().path_names[converted_clicked_directory_id].to_string();
    let selected_splitted_path: Vec<&str> = selected_full_path.split_terminator("\\").collect();
    let file_or_dir_new_name: String = NEW_FILE_OR_DIR_NAME.lock().unwrap().clone();
    let selected_new_path: String = get_restructured_path(&selected_full_path, selected_splitted_path, &file_or_dir_new_name);

    match std::fs::rename(&selected_full_path, &selected_new_path) {
        Ok(_) => {
            let _ = std::mem::replace(&mut files.write().path_names[converted_clicked_directory_id], format!("{}", selected_new_path));
            files.write().reload_path_list();
        },
        Err(error) => panic!("{}", error)
    }
}

fn get_restructured_path<'a>(selected_full_path: &'a String, selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    return match std::fs::metadata(selected_full_path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                restructure_dir_path(selected_splitted_path, file_or_dir_new_name)
            } else if path_metadata.is_file() {
                restructure_file_path(selected_splitted_path, file_or_dir_new_name)
            } else {
                "None".to_string()
            }
        },
        Err(error) => panic!("{}", error)
    }
}

fn restructure_dir_path<'a>(mut selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    let mut restructured_dir_path: String = String::new();
    selected_splitted_path.pop();
    selected_splitted_path.push(file_or_dir_new_name.as_str());

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        if index == 0 {
            restructured_dir_path.push_str(splitted_part);
        } else {
            restructured_dir_path.push_str(format!("\\{}", splitted_part).as_str());
        }
    }
    return restructured_dir_path;
}

fn restructure_file_path<'a>(mut selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    let mut restructured_file_path: String = String::new();
    let file_extension_last_occurrence_index = selected_splitted_path.last().unwrap().to_string().rfind(".");
    let (_, file_extension) = selected_splitted_path.last().unwrap().split_at(file_extension_last_occurrence_index.unwrap());
    selected_splitted_path.pop();
    selected_splitted_path.push(file_or_dir_new_name.as_str());

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        if index == 0 {
            restructured_file_path.push_str(splitted_part);
        } else {
            restructured_file_path.push_str(format!("\\{}", splitted_part).as_str());
        }
    }
    restructured_file_path.push_str(file_extension);
    return restructured_file_path;
}
