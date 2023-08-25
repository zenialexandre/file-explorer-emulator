use std::ops::Index;
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
        let mut selected_splitted_path: Vec<&str> = selected_full_path.split_terminator("\\").collect();
        let mut file_or_dir_new_name: String = "bomdiateste.txt".to_string();

        let mut selected_new_path: String = get_restructured_path(selected_splitted_path, &file_or_dir_new_name);

        println!("Path name: {}", files.read().path_names[converted_clicked_directory_id].to_string());
        /*println!("Path stack: {}", files.read().path_stack[1].to_string());
        println!("Path stack: {}", files.read().path_stack[4].to_string());
        println!("{}", selected_splitted_path[1].to_string());*/

        println!("{}", selected_full_path);
        println!("{}", selected_new_path);

        match std::fs::rename(&selected_full_path, &selected_new_path) {
            Ok(_) => {
                let new = std::mem::replace(&mut files.write().path_names[converted_clicked_directory_id], format!("{}", selected_new_path));
                //let _ = files.write().path_names[converted_clicked_directory_id].replace();
                files.write().reload_path_list();
            },
            Err(error) => panic!("{}", error)
        }
    }
}

fn get_restructured_path<'a>(mut selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    let mut restructured_path: String = String::new();

    selected_splitted_path.pop();
    selected_splitted_path.push(file_or_dir_new_name.as_str());

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        println!("{}", index.to_string());
        println!("{}", splitted_part.to_string());
    }

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        if index == 0 {
            restructured_path.push_str(splitted_part);
        } else {
            restructured_path.push_str(format!("\\{}", splitted_part).as_str());
        }
    }
    println!("{}", restructured_path.to_string());
    return restructured_path.to_string()
}
