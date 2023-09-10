use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use dioxus::hooks::UseRef;
use crate::{Files, REGULAR_FILE, window_helper};

pub fn execute_copy_operation(new_file_or_dir_name: &Mutex<String>, copied_file_or_dir_name: &Mutex<Vec<String>>, clicked_directory_id: &Mutex<usize>, files: &UseRef<Files>) {
    *new_file_or_dir_name.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id);
    *copied_file_or_dir_name.lock().unwrap() = new_file_or_dir_name.lock().unwrap().split("\\").map(|element| element.to_string()).collect();
}

pub fn execute_paste_operation(new_file_or_dir_name: &Mutex<String>, copied_file_or_dir_name: &Mutex<Vec<String>>, files: &UseRef<Files>) {
    let selected_current_stack = window_helper::get_selected_current_stack(files);

    if window_helper::get_file_type_formatted(new_file_or_dir_name.lock().unwrap().to_string()) == REGULAR_FILE.to_string() {
        paste_file(selected_current_stack, new_file_or_dir_name, copied_file_or_dir_name, files);
    } else {
        paste_dir();
    }
}

fn paste_file(mut selected_current_stack: String, new_file_or_dir_name: &Mutex<String>, copied_file_or_dir_name: &Mutex<Vec<String>>, files: &UseRef<Files>) {
    let current_file = copied_file_or_dir_name.lock().unwrap().last().expect("Parse to string.").to_string();
    selected_current_stack.push_str(format!("\\{}", current_file).as_str());
    let mut new_file = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
    let original_file = File::open(new_file_or_dir_name.lock().unwrap().as_str());
    let mut buffer: String = String::new();
    original_file.unwrap().read_to_string(&mut buffer).expect("Unable to read.");
    new_file.write_all(buffer.as_bytes()).expect("Unable to write.");
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn paste_dir() {

}
