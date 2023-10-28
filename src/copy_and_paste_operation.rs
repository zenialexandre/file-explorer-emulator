use std::io;
use std::fs::File;
use std::ops::Not;
use std::string::ToString;
use std::sync::Mutex;
use dioxus::hooks::UseRef;
use fs_extra::dir::CopyOptions;

use crate::Files;
use crate::{conflict_process, create_operation, cut_operation, window_helper};
use crate::{PREVIOUS_OPERATION_DONE, REGULAR_FILE};

lazy_static! { pub(crate) static ref COPIED_FILE_OR_DIR_NAME: Mutex<Vec<String>> = Mutex::new(Vec::new()); }
lazy_static! { pub(crate) static ref COPY_INCREMENTAL_ID: Mutex<u32> = Mutex::new(0); }

pub(crate) fn execute_copy_operation(files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    *COPIED_FILE_OR_DIR_NAME.lock().unwrap() = window_helper::get_selected_full_path(files, clicked_directory_id)
        .split("\\").map(|element| element.to_string()).collect();
    *PREVIOUS_OPERATION_DONE.lock().unwrap() = "Copy".to_string();
}

pub(crate) fn execute_paste_operation(files: &UseRef<Files>, previous_operation_done: &Mutex<String>) {
    let copied_file_or_dir_name_joined: String = COPIED_FILE_OR_DIR_NAME.lock().unwrap().join("\\");
    let selected_current_stack: String = window_helper::get_selected_current_stack(files);

    paste_operation(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), files, previous_operation_done);
    files.write().path_names.push(selected_current_stack.clone());
    files.write().reload_path_list();
}

fn paste_operation(selected_current_stack: String, copied_file_or_dir_name_joined: String, files: &UseRef<Files>, previous_operation_done: &Mutex<String>) {
    if window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string() {
        paste_file(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), files, previous_operation_done);
    } else {
        paste_dir(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone(), files, previous_operation_done);
    }
}

fn paste_file(selected_current_stack: String, copied_file_or_dir_name_joined: String, files: &UseRef<Files>, previous_operation_done: &Mutex<String>) {
    let mut file_name: String = COPIED_FILE_OR_DIR_NAME.lock().unwrap().last().unwrap().to_string();

    if conflict_process::check_file_or_dir_conflict(file_name.clone(), selected_current_stack.clone(), files)
        && window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string() {
        file_name = get_restructured_file_name(file_name);
    }
    end_paste_file_operation(selected_current_stack.clone(), file_name,
                             copied_file_or_dir_name_joined.clone(), previous_operation_done);
}

fn get_restructured_file_name(file_name: String) -> String {
    *COPY_INCREMENTAL_ID.lock().unwrap() += 1;
    let file_name_with_extension: &str = file_name.as_str();
    let file_extension_last_occurrence_index: usize = file_name_with_extension.rfind(".").unwrap();
    let (splitted_file_name, splitted_file_extension) = file_name_with_extension.split_at(file_extension_last_occurrence_index);
     format!("{splitted_file_name} Copy {copy_index}{splitted_file_extension}",
             splitted_file_name = splitted_file_name, copy_index = COPY_INCREMENTAL_ID.lock().unwrap(), splitted_file_extension = splitted_file_extension)
}

fn end_paste_file_operation(mut selected_current_stack: String, file_name: String, copied_file_or_dir_name_joined: String,
                            previous_operation_done: &Mutex<String>) {
    selected_current_stack.push_str(format!("\\{}", file_name.clone()).as_str());

    if previous_operation_done.lock().unwrap().eq_ignore_ascii_case("Cut") {
        cut_operation::rename_on_cut(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    } else {
        let mut new_file: File = File::create(selected_current_stack.clone()).unwrap_or_else(|error| panic!("{}", error));
        let original_file = File::open(copied_file_or_dir_name_joined.as_str());
        io::copy(&mut original_file.unwrap(), &mut new_file).unwrap_or_else(|error| panic!("{}", error));
    }
}

fn paste_dir(mut selected_current_stack: String, copied_file_or_dir_name_joined: String, files: &UseRef<Files>, previous_operation_done: &Mutex<String>) {
    let copy_options: CopyOptions = CopyOptions::new();

    if conflict_process::check_file_or_dir_conflict(copied_file_or_dir_name_joined.split("\\").last().unwrap().to_string(),
                                                    selected_current_stack.clone(), files)
        && (window_helper::get_file_type_formatted(copied_file_or_dir_name_joined.clone()) == REGULAR_FILE.to_string()).not() {
        paste_dir_with_conflict(selected_current_stack.clone(),
                                copied_file_or_dir_name_joined.clone(), previous_operation_done, &copy_options);
    } else {
        if previous_operation_done.lock().unwrap().eq_ignore_ascii_case("Cut") {
            selected_current_stack.push_str(
                format!("\\{}", copied_file_or_dir_name_joined.as_str().split("\\").last().unwrap()).as_str());
            cut_operation::rename_on_cut(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
        } else {
            fs_extra::dir::copy(copied_file_or_dir_name_joined, selected_current_stack.clone(), &copy_options)
                .unwrap_or_else(|error| panic!("{}", error));
        }
    }
}

fn paste_dir_with_conflict(mut selected_current_stack: String, copied_file_or_dir_name_joined: String, previous_operation_done: &Mutex<String>,
                           copy_options: &CopyOptions) {
    *COPY_INCREMENTAL_ID.lock().unwrap() += 1;
    let mut dir_name_without_conflict = copied_file_or_dir_name_joined.split("\\").last().unwrap().to_string();
    dir_name_without_conflict.push_str(format!(" Copy {copy_index}", copy_index = COPY_INCREMENTAL_ID.lock().unwrap()).as_str());
    selected_current_stack.push_str(format!("\\{}", dir_name_without_conflict).as_str());

    if previous_operation_done.lock().unwrap().eq_ignore_ascii_case("Cut") {
        cut_operation::rename_on_cut(selected_current_stack.clone(), copied_file_or_dir_name_joined.clone());
    } else {
        create_operation::add_new_dir(selected_current_stack.clone(), false);
        fs_extra::copy_items(&vec![copied_file_or_dir_name_joined], selected_current_stack.clone(), copy_options)
            .unwrap_or_else(|error| panic!("{}", error));
    }
}
