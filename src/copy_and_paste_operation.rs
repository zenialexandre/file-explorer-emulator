use std::{io};
use std::fs::File;
use std::sync::Mutex;
use dioxus::core::Scope;
use dioxus::hooks::UseRef;
use dioxus::prelude::*;
use fs_extra::dir::CopyOptions;
use crate::{conflict_process, Files, REGULAR_FILE, window_helper};

lazy_static! { pub static ref COPIED_FILE_OR_DIR_NAME: Mutex<Vec<String>> = Mutex::new(Vec::new()); }

pub fn execute_copy_operation(clicked_directory_id: &Mutex<usize>, files: &UseRef<Files>) {
    *COPIED_FILE_OR_DIR_NAME.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id)
        .split("\\").map(|element| element.to_string()).collect();
}

pub fn execute_paste_operation(cx: Scope, files: &UseRef<Files>) {
    let copied_file_or_dir_name_joined = COPIED_FILE_OR_DIR_NAME.lock().unwrap().join("\\");
    let selected_current_stack = window_helper::get_selected_current_stack(files);

    if conflict_process::check_file_or_dir_conflict(selected_current_stack.clone(), files) {
        conflict_process::with_conflict_process(cx, files, selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    } else {
        no_conflict_paste_process(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    }
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn no_conflict_paste_process(selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    if window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string() {
        paste_file(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    } else {
        paste_dir(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    }
}

fn paste_file(mut selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    selected_current_stack.push_str(format!("\\{}", COPIED_FILE_OR_DIR_NAME.lock().unwrap().last().unwrap()).as_str());
    let mut new_file = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
    let mut original_file = File::open(copied_file_or_dir_name_joined.as_str());
    io::copy(&mut original_file.unwrap(), &mut new_file).unwrap_or_else(|error| panic!("{}", error));
}

fn paste_dir(selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    let copy_options = CopyOptions::new();
    fs_extra::dir::copy(copied_file_or_dir_name_joined, selected_current_stack, &copy_options)
        .unwrap_or_else(|error| panic!("{}", error));
}
