use std::sync::Mutex;
use dioxus::prelude::{Scope, UseRef};
use crate::{CLICKED_DIRECTORY_ID, Files};

pub fn create_global_shortcuts(context: &Scope, files: &UseRef<Files>) {
    let path = &files.read().path_names[get_converted_usize_from_string(CLICKED_DIRECTORY_ID.lock().unwrap().to_string())];

    dioxus_desktop::use_global_shortcut(context, "ctrl+r", {
        move || {
            rename_action(path, &CLICKED_DIRECTORY_ID);
        }
    });
}

fn rename_action(path: &String, clicked_directory_id: &Mutex<usize>) {
    if get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string())
        >= get_converted_usize_from_string("0".to_string()) {
        let metadata = std::fs::metadata(path.to_string());
        println!("{}", path.to_string());
    }
}

fn get_converted_usize_from_string(any_string: String) -> usize {
    return any_string.parse().unwrap();
}
