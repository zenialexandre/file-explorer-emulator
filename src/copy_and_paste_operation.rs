use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use dioxus::hooks::UseRef;
use crate::{Files, REGULAR_FILE, window_helper};

lazy_static! { static ref COPIED_FILE_OR_DIR_NAME: Mutex<Vec<String>> = Mutex::new(Vec::new()); }

pub fn execute_copy_operation(clicked_directory_id: &Mutex<usize>, files: &UseRef<Files>) {
    *COPIED_FILE_OR_DIR_NAME.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id)
        .split("\\").map(|element| element.to_string()).collect();
}

pub fn execute_paste_operation(files: &UseRef<Files>) {
    let copied_file_or_dir_name_joined = COPIED_FILE_OR_DIR_NAME.lock().unwrap().join("\\");
    let selected_current_stack = window_helper::get_selected_current_stack(files);

    if window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string() {
        paste_file(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    } else {
        paste_dir();
    }
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn paste_file(mut selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    selected_current_stack.push_str(format!("\\{}", COPIED_FILE_OR_DIR_NAME.lock().unwrap().last().unwrap()).as_str());
    let new_file = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
    let original_file = File::open(copied_file_or_dir_name_joined.as_str());
    copy_content(original_file.unwrap(), new_file);
}

fn paste_dir() {
    // todo -> Verify if a directory has content inside, and paste it all
}

fn copy_content(mut original_file: File, mut new_file: File) {
    io::copy(&mut original_file, &mut new_file).unwrap_or_else(|error| panic!("{}", error));
}
