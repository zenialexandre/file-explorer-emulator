use std::sync::Mutex;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::Files;
use crate::window_helper::get_converted_usize_from_string;

pub fn rename_event(context: Scope, files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    fire_rename_popup(context);
    let converted_clicked_directory_id: usize = get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());
    let selected_full_path: String = files.read().path_names[converted_clicked_directory_id].to_string();
    let selected_splitted_path: Vec<&str> = selected_full_path.split_terminator("\\").collect();
    let file_or_dir_new_name: String = "bomdiateste".to_string();
    let selected_new_path: String = get_restructured_path(&selected_full_path, selected_splitted_path, &file_or_dir_new_name);

    match std::fs::rename(&selected_full_path, &selected_new_path) {
        Ok(_) => {
            let _ = std::mem::replace(&mut files.write().path_names[converted_clicked_directory_id], format!("{}", selected_new_path));
            files.write().reload_path_list();
        },
        Err(error) => panic!("{}", error)
    }
}

fn fire_rename_popup(context: Scope) {
    let window = dioxus_desktop::use_window(context);
    let dom = VirtualDom::new(rename_popup);
    window.new_window(dom, Config::default()
        .with_window(WindowBuilder::new()));
}

fn rename_popup(context: Scope) -> Element {
    context.render(rsx! {
        div { "popup" }
    })
}

fn get_restructured_path<'a>(selected_full_path: &'a String, selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    return match std::fs::metadata(selected_full_path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                restructure_dir_path(selected_splitted_path, file_or_dir_new_name)
            } else if path_metadata.is_file() {
                restructure_file_path(selected_splitted_path, file_or_dir_new_name)
            } else {
                "None".to_string()
            }
        },
        Err(error) => panic!("{}", error)
    }
}

fn restructure_dir_path<'a>(mut selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    let mut restructured_dir_path: String = String::new();
    selected_splitted_path.pop();
    selected_splitted_path.push(file_or_dir_new_name.as_str());

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        if index == 0 {
            restructured_dir_path.push_str(splitted_part);
        } else {
            restructured_dir_path.push_str(format!("\\{}", splitted_part).as_str());
        }
    }
    return restructured_dir_path;
}

fn restructure_file_path<'a>(mut selected_splitted_path: Vec<&'a str>, file_or_dir_new_name: &'a String) -> String {
    let mut restructured_file_path: String = String::new();
    let file_extension_last_occurrence_index = selected_splitted_path.last().unwrap().to_string().rfind(".");
    let (_, file_extension) = selected_splitted_path.last().unwrap().split_at(file_extension_last_occurrence_index.unwrap());
    selected_splitted_path.pop();
    selected_splitted_path.push(file_or_dir_new_name.as_str());

    for (index, splitted_part) in selected_splitted_path.iter().enumerate() {
        if index == 0 {
            restructured_file_path.push_str(splitted_part);
        } else {
            restructured_file_path.push_str(format!("\\{}", splitted_part).as_str());
        }
    }
    restructured_file_path.push_str(file_extension);
    return restructured_file_path;
}