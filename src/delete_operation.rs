use std::sync::Mutex;
use dioxus::hooks::UseRef;
use dioxus::prelude::*;

use crate::Files;
use crate::{window_helper, delete_operation};
use crate::{CLICKED_DIRECTORY_ID, GENERIC_POPUP_ID};

pub(crate) fn execute_delete_operation(files_props: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    let selected_full_path: String = window_helper::get_selected_full_path(files_props, clicked_directory_id);

    match std::fs::metadata(selected_full_path.clone()) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                std::fs::remove_dir_all(selected_full_path.as_str()).expect("Delete Directory");
            } else if path_metadata.is_file() {
                std::fs::remove_file(selected_full_path.as_str()).expect("Delete File");
            }
            files_props.write().path_names.pop();
            files_props.write().reload_path_list();
        },
        Err(error) => panic!("{}", error.to_string())
    }
}

#[inline_props]
pub(crate) fn delete_popup(cx: Scope, files_props: UseRef<Files>) -> Element {
    GENERIC_POPUP_ID.lock().unwrap().push(dioxus_desktop::use_window(cx).id());

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
