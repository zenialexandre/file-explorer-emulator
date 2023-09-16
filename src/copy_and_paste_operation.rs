use std::{io};
use std::fs::{File};
use std::string::ToString;
use std::sync::{Mutex};
use dioxus::core::Scope;
use dioxus::hooks::{UseRef};
use fs_extra::dir::CopyOptions;
use crate::{conflict_process, Files, REGULAR_FILE, window_helper};

lazy_static! { pub(crate) static ref COPIED_FILE_OR_DIR_NAME: Mutex<Vec<String>> = Mutex::new(Vec::new()); }
lazy_static! { pub(crate) static ref COPY_INCREMENTAL_ID: Mutex<u32> = Mutex::new(0); }

pub(crate) fn execute_copy_operation(clicked_directory_id: &Mutex<usize>, files: &UseRef<Files>) {
    *COPIED_FILE_OR_DIR_NAME.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id)
        .split("\\").map(|element| element.to_string()).collect();
}

pub(crate) fn execute_paste_operation(cx: Scope, files: &UseRef<Files>) {
    let copied_file_or_dir_name_joined = COPIED_FILE_OR_DIR_NAME.lock().unwrap().join("\\");
    let selected_current_stack = window_helper::get_selected_current_stack(files);

    paste_operation(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), cx, files);
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn paste_operation(selected_current_stack: String, copied_file_or_dir_name_joined: String, cx: Scope, files: &UseRef<Files>) {
    if window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string() {
        paste_file(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), files);
    } else {
        paste_dir(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), cx, files);
    }
}

fn paste_file(selected_current_stack: String, copied_file_or_dir_name_joined: String, files: &UseRef<Files>) {
    let mut file_name = COPIED_FILE_OR_DIR_NAME.lock().unwrap().last().unwrap().to_string();;

    if conflict_process::check_file_or_dir_conflict(selected_current_stack.clone(), files) {
        file_name = get_restructured_file_name(file_name);
    }
    end_paste_file_operation(selected_current_stack.clone(), file_name, copied_file_or_dir_name_joined.clone());
}

fn get_restructured_file_name(file_name: String) -> String {
    *COPY_INCREMENTAL_ID.lock().unwrap() += 1;
    let file_name_with_extension = file_name.as_str();
    let file_extension_last_occurrence_index = file_name_with_extension.rfind(".").unwrap();
    let (splitted_file_name, splitted_file_extension) = file_name_with_extension.split_at(file_extension_last_occurrence_index);
     format!("{splitted_file_name} Copy {copy_index}{splitted_file_extension}",
             splitted_file_name = splitted_file_name, copy_index = COPY_INCREMENTAL_ID.lock().unwrap(), splitted_file_extension = splitted_file_extension)
}

fn end_paste_file_operation(mut selected_current_stack: String, file_name: String, copied_file_or_dir_name_joined: String) {
    selected_current_stack.push_str(format!("\\{}", file_name.clone()).as_str());
    let mut new_file = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
    let original_file = File::open(copied_file_or_dir_name_joined.as_str());
    io::copy(&mut original_file.unwrap(), &mut new_file).unwrap_or_else(|error| panic!("{}", error));
}

fn paste_dir(selected_current_stack: String, copied_file_or_dir_name_joined: String, cx: Scope, files: &UseRef<Files>) {
    let copy_options = CopyOptions::new();

    if conflict_process::check_file_or_dir_conflict(selected_current_stack.clone(), files) {
        // todo
    } else {
        fs_extra::dir::copy(copied_file_or_dir_name_joined, selected_current_stack.clone(), &copy_options)
            .unwrap_or_else(|error| panic!("{}", error));
    }
}
