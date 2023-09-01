use std::sync::Mutex;
use dioxus::prelude::UseRef;

use crate::{Files, window_helper};

pub fn execute_create_operation(files: &UseRef<Files>, new_file_or_dir_name: &Mutex<String>) {
    let mut selected_current_stack: String = window_helper::get_selected_current_stack(files);
    selected_current_stack.push_str(format!("\\{}", new_file_or_dir_name.lock().unwrap()).as_str());
    add_new_path(files, selected_current_stack);
}

fn add_new_path(files: &UseRef<Files>, selected_current_stack: String) {
    files.write().path_names.push(selected_current_stack);
    files.write().reload_path_list();
}
