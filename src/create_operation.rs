use std::sync::Mutex;
use dioxus::prelude::UseRef;

use crate::{Files, window_helper};

pub fn execute_create_operation(files: &UseRef<Files>, new_file_or_dir_name: &Mutex<String>) {
    let is_recursive_dir_input: bool = is_recursive_dir(new_file_or_dir_name.lock().unwrap().as_str().trim());
    let mut selected_current_stack: String = window_helper::get_selected_current_stack(files);
    selected_current_stack.push_str(format!("\\{}", new_file_or_dir_name.lock().unwrap()).as_str().trim());
    add_new_path(files, selected_current_stack, is_recursive_dir_input);
}

fn is_recursive_dir(new_file_or_dir_name: &str) -> bool {
    new_file_or_dir_name.contains("/")
}

fn add_new_path(files: &UseRef<Files>, selected_current_stack: String, is_recursive_dir_input: bool) {
    match is_recursive_dir_input {
        true => std::fs::create_dir(selected_current_stack.clone()),
        false => std::fs::create_dir_all(selected_current_stack.clone()), // todo
    }.expect("It should create a new directory.");
    files.write().path_names.push(selected_current_stack);
    files.write().reload_path_list();
}
