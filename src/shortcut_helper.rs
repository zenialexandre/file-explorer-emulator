use std::sync::Mutex;
use dioxus::prelude::Scope;

pub fn create_global_shortcuts(context: &Scope, file_path: &Mutex<String>, clicked_directory_id: &Mutex<usize>) {
    dioxus_desktop::use_global_shortcut(context, "ctrl+r", {
        move || {
            println!("bom dia");
        }
    });
}
