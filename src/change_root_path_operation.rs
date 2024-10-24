use dioxus::prelude::*;
use winapi::shared::minwindef::DWORD;

use crate::Files;
use crate::{ROOT_PATH, GENERIC_POPUP_ID};

fn execute_change_root_path_operation(selected_root_path: Signal<String>, files_props: &mut Signal<Files>) {
    if !selected_root_path.read().is_empty() {
        *ROOT_PATH.lock().unwrap() = selected_root_path.read().to_string();
        files_props.write().path_stack.clear();
        files_props.write().path_names.clear();
        files_props.write().path_stack.push(ROOT_PATH.lock().unwrap().to_string());
        files_props.write().reload_path_list();
    }
}

fn get_available_devices_paths() -> Vec<String> {
    unsafe {
        let bitmask: DWORD = winapi::um::fileapi::GetLogicalDrives();
        let mut available_devices_paths: Vec<String> = Vec::new();

        for bit in 0..26 {
            if (bitmask & (1 << bit)) != 0 {
                let drive_letter = (b'A' + bit as u8) as char;
                available_devices_paths.push(format!("{}://", drive_letter));
            }
        }
        available_devices_paths
    }
}

#[inline_props]
pub(crate) fn change_root_path_popup(files_props: Signal<Files>) -> Element {
    let mut selected_root_path: Signal<String> = use_signal(|| "".to_string());
    let available_devices_paths: Signal<Vec<String>> = use_signal(|| get_available_devices_paths());
    GENERIC_POPUP_ID.lock().unwrap().push(dioxus::desktop::use_window().id());

    rsx! {
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", },
            style { { include_str!("./assets/change_root_path_popup.css") } },
            div {
                class: "central-div",
                h1 { "Choose a different root path, from the available devices: " }
                br {}
                div {
                    select {
                        class: "devices-combobox",
                        multiple: true,
                        oninput: move |select_event: Event<FormData>| {
                            selected_root_path.set(select_event.value().to_string());
                        },
                        { create_available_devices_paths_combobox(available_devices_paths) }
                    }
                },
                br { }
                i {
                    class: "material-icons",
                    onclick: move |_| {
                        dioxus::desktop::use_window().close();
                    },
                    "cancel"
                },
                span { }
                i {
                    class: "material-icons",
                    onclick: move |_| {
                        execute_change_root_path_operation(selected_root_path, &mut files_props.clone());
                        dioxus::desktop::use_window().close();
                    },
                    "check_circle"
                }
            }
        }
    }
}

fn create_available_devices_paths_combobox(available_devices_paths: Signal<Vec<String>>) -> Element {
    rsx!({
        available_devices_paths.read().clone().into_iter().enumerate().map(|(_, path)| {
            rsx!(
                option {
                    class: "devices-option",
                    value: "{path}",
                    label: "{path}"
                }
            )}
        )}
    )
}
