use dioxus::prelude::*;
use crate::{Files};

pub(crate) fn check_file_or_dir_conflict(file_or_dir_name: String, mut selected_current_stack: String, files: &UseRef<Files>) -> bool {
    selected_current_stack.push_str(format!("\\{}", file_or_dir_name).as_str());

    if files.read().path_names.contains(&selected_current_stack) {
        return true;
    }
    return false;
}
