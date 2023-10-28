use std::fs::File;
use std::ops::Not;
use std::sync::Mutex;
use dioxus::hooks::UseState;
use dioxus::prelude::*;

use crate::Files;
use crate::{conflict_popup, conflict_popupProps};
use crate::{conflict_process, window_helper, rename_operation};
use crate::{NEW_FILE_OR_DIR_NAME, CLICKED_DIRECTORY_ID, GENERIC_POPUP_ID};

pub(crate) fn execute_create_operation(files: &UseRef<Files>, new_file_or_dir_name: &Mutex<String>, enable_file_creation: &UseState<bool>) -> bool {
    let selected_current_stack: String = window_helper::get_selected_current_stack(files);

    if conflict_process::check_file_or_dir_conflict(new_file_or_dir_name.lock().unwrap().to_string(), selected_current_stack.clone(), files) {
        return false;
    } else {
        create_process(selected_current_stack.clone(), new_file_or_dir_name, enable_file_creation);
    }
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
    return true;
}

fn create_process(mut selected_current_stack: String, new_file_or_dir_name: &Mutex<String>, enable_file_creation: &UseState<bool>) {
    if enable_file_creation.get() == &true {
        verify_extension(selected_current_stack.clone(), new_file_or_dir_name);
    } else {
        selected_current_stack.push_str(format!("\\{}", new_file_or_dir_name.lock().unwrap()).as_str().trim());
        add_new_dir(selected_current_stack.clone(), is_recursive_dir(new_file_or_dir_name.lock().unwrap().as_str().trim()));
    }
}

fn verify_extension(mut selected_current_stack: String, new_file_or_dir_name: &Mutex<String>) {
    if new_file_or_dir_name.lock().unwrap().contains(".").not() {
        new_file_or_dir_name.lock().unwrap().push_str(".txt");
    }

    selected_current_stack.push_str(format!("\\{}", new_file_or_dir_name.lock().unwrap()).as_str().trim());
    match File::create(selected_current_stack.clone()) {
        Ok(_) => println!(),
        Err(error) => println!("{}", error)
    }
    add_new_file(selected_current_stack.clone());
}

fn add_new_file(selected_current_stack: String) {
    match std::fs::OpenOptions::new().write(true).create_new(true).open(selected_current_stack.split("\\").last().unwrap()) {
        Ok(_) => println!(),
        Err(error) => println!("{}", error)
    }
}

fn is_recursive_dir(new_file_or_dir_name: &str) -> bool {
    new_file_or_dir_name.contains("/") || new_file_or_dir_name.contains("\\")
}

pub(crate) fn add_new_dir(selected_current_stack: String, is_recursive_dir_input: bool) {
    match is_recursive_dir_input {
        true => std::fs::create_dir_all(selected_current_stack.clone()),
        false => std::fs::create_dir(selected_current_stack.clone()),
    }.unwrap_or_else(|error| println!("Error on create_operation: {}", error));
}

#[inline_props]
pub(crate) fn create_rename_popup<'a>(cx: Scope, files_props: UseRef<Files>, title_props: &'a str) -> Element {
    GENERIC_POPUP_ID.lock().unwrap().push(dioxus_desktop::use_window(cx).id());
    let enable_file_creation: &UseState<bool> = use_state(cx, || false);

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
                        autofocus: "true",
                        r#type: "text",
                        placeholder: "Directory/File new name",
                        id: "directory-file-name",
                        oninput: |type_event: Event<FormData>| {
                            *NEW_FILE_OR_DIR_NAME.lock().unwrap() = type_event.value.to_string()
                        }
                    },
                    if title_props == &"Create" {
                        rsx!(
                            br {}, br {},
                            label {
                                input {
                                    r#type: "checkbox",
                                    checked: "{enable_file_creation}",
                                    id: "enable-file-creation",
                                    oninput: move |check_event: Event<FormData>| {
                                        enable_file_creation.set(check_event.value.parse().unwrap());
                                    }
                                }
                                "Check if the new content is a file."
                            },
                            br {},
                        )
                    }
                    br {},
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
                                    "Create" => {
                                        if execute_create_operation(files_props, &NEW_FILE_OR_DIR_NAME, enable_file_creation).not() {
                                            let conflict_dom: VirtualDom = VirtualDom::new_with_props(conflict_popup, conflict_popupProps
                                                { files_props: files_props.clone(), enable_file_creation_props: enable_file_creation.clone() });
                                            window_helper::create_new_dom_generic_window_state(cx, conflict_dom, "Conflict");
                                        }
                                    },
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
