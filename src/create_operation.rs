use std::fs::File;
use std::ops::Not;
use std::sync::Mutex;
use dioxus::hooks::UseState;
use dioxus::prelude::UseRef;

use crate::{conflict_process, Files, window_helper};

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
    let _ = File::create(selected_current_stack.clone());
    add_new_file(selected_current_stack.clone());
}

fn add_new_file(selected_current_stack: String) {
    let _ = std::fs::OpenOptions::new().write(true).create_new(true).open(selected_current_stack.split("\\").last().unwrap());
}

fn is_recursive_dir(new_file_or_dir_name: &str) -> bool {
    new_file_or_dir_name.contains("/") || new_file_or_dir_name.contains("\\")
}

pub(crate) fn add_new_dir(selected_current_stack: String, is_recursive_dir_input: bool) {
    match is_recursive_dir_input {
        true => std::fs::create_dir_all(selected_current_stack.clone()),
        false => std::fs::create_dir(selected_current_stack.clone()),
    }.unwrap_or_else(|error| panic!("Error on create_operation: {}", error));
}
