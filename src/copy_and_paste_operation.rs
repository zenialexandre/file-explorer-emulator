use std::fs::File;
use std::sync::Mutex;
use dioxus::hooks::UseRef;
use crate::{Files, REGULAR_FILE, window_helper};

pub fn execute_copy_operation(new_file_or_dir_name: &Mutex<String>, copied_file_or_dir_name: &Mutex<String>, clicked_directory_id: &Mutex<usize>, files: &UseRef<Files>) {
    new_file_or_dir_name.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id);
    copied_file_or_dir_name.lock().unwrap() = new_file_or_dir_name.lock().unwrap().split("\\").map(|element| element.to_string()).collect();
}

pub fn execute_paste_operation(new_file_or_dir_name: &Mutex<String>, copied_file_or_dir_name: &Mutex<String>, files: &UseRef<Files>) {
    let mut selected_current_stack = window_helper::get_selected_current_stack(files);

    if window_helper::get_file_type_formatted(new_file_or_dir_name.lock().unwrap().to_string()) == REGULAR_FILE.to_string() {
        let current_file = copied_file_or_dir_name.lock().unwrap().last().expect("Parse to string.").to_string();
        selected_current_stack.push_str(format!("\\{}", current_file).as_str());
        let mut new_file = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
        let new_file_metadata = new_file.metadata();
        new_file_metadata.expect("Set ReadOnly").permissions().set_readonly(false);
        //std::fs::copy(current_file, created_new_file);
        files.write().path_names.push(selected_current_stack.clone());
        files.write().reload_path_list();
    } else {
        // todo
    }
}
