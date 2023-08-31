use std::sync::Mutex;
use dioxus::hooks::UseRef;

use crate::Files;
use crate::window_helper::get_selected_full_path;

pub fn execute_delete_operation(files_props: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    let selected_full_path: String = get_selected_full_path(files_props, clicked_directory_id);

    match std::fs::metadata(selected_full_path.clone()) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                std::fs::remove_dir_all(selected_full_path.as_str()).expect("Delete Directory");
            } else if path_metadata.is_file() {
                println!("is file");
                std::fs::remove_file(selected_full_path.as_str()).expect("Delete File");
            }
            files_props.write().path_names.pop();
            files_props.write().reload_path_list();
        },
        Err(error) => panic!("{}", error.to_string())
    }
}
