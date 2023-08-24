use std::sync::Mutex;
use dioxus::core::Event;
use dioxus::events::KeyboardData;
use dioxus::hooks::UseRef;
use dioxus::html::input_data::keyboard_types::{Code, Modifiers};
use crate::Files;
use crate::window_helper::get_converted_usize_from_string;

pub fn handle_keydown_event(keydown_event: Event<KeyboardData>, files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyR {
        let converted_clicked_directory_id: usize = get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());
        let selected_full_path: String = files.read().path_names[converted_clicked_directory_id].to_string();
        let selected_splitted_path: Vec<&str> = vec![selected_full_path.split_terminator("\\").collect()]; 

        println!("Path name: {}", files.read().path_names[converted_clicked_directory_id].to_string());
        println!("Path stack: {}", files.read().path_stack[1].to_string());
        println!("Path stack: {}", files.read().path_stack[4].to_string());
        println!("{}", selected_splitted_path[1].to_string());

        match std::fs::rename(selected_splitted_path[1], "bomdiateste") {
            Ok(_) => {
                let _ = std::mem::replace(&mut files.read().path_names[converted_clicked_directory_id], format!("{}{}", selected_splitted_path[0], "bomdiateste".to_string()));
                //let _ = files.write().path_names[converted_clicked_directory_id].replace();
                files.write().reload_path_list();
            },
            Err(error) => panic!("{}", error)
        }
    }
}
