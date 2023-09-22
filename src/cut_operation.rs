use std::sync::Mutex;
use dioxus::prelude::UseRef;

use crate::Files;
use crate::copy_and_paste_operation;
use crate::PREVIOUS_OPERATION_DONE;

pub(crate) fn execute_cut_operation(files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    copy_and_paste_operation::execute_copy_operation(files, clicked_directory_id);
    *PREVIOUS_OPERATION_DONE.lock().unwrap() = "Cut".to_string();
}

pub(crate) fn rename_on_cut(selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    match std::fs::rename(copied_file_or_dir_name_joined.clone(), selected_current_stack.clone()) {
        Ok(_) => { },
        Err(error) => panic!("{}", error)
    }
}
