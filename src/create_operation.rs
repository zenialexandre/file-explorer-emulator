use std::fs::File;
use std::sync::Mutex;
use dioxus::prelude::UseRef;

use crate::{Files, window_helper};

pub(crate) fn execute_create_operation(files: &UseRef<Files>, new_file_or_dir_name: &Mutex<String>) {
    let mut selected_current_stack: String = window_helper::get_selected_current_stack(files);
    selected_current_stack.push_str(format!("\\{}", new_file_or_dir_name.lock().unwrap()).as_str().trim());

    if new_file_or_dir_name.lock().unwrap().contains(".") {
        let _ = File::create(selected_current_stack.clone());
        add_new_file(selected_current_stack.clone());
    } else {
        add_new_dir(selected_current_stack.clone(), is_recursive_dir(new_file_or_dir_name.lock().unwrap().as_str().trim()));
    }
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn add_new_file(selected_current_stack: String) {
    let _ = std::fs::OpenOptions::new().write(true).create_new(true).open(selected_current_stack.split("\\").last().unwrap());
}

fn is_recursive_dir(new_file_or_dir_name: &str) -> bool {
    new_file_or_dir_name.contains("/") || new_file_or_dir_name.contains("\\")
}

fn add_new_dir(selected_current_stack: String, is_recursive_dir_input: bool) {
    match is_recursive_dir_input {
        true => std::fs::create_dir_all(selected_current_stack.clone()),
        false => std::fs::create_dir(selected_current_stack.clone()),
    }.unwrap_or_else(|error| panic!("Error on create_operation: {}", error));
}
