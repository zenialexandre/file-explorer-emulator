use dioxus::prelude::*;
use dioxus_desktop::tao::window::Icon as TaoIcon;
use image::GenericImageView;
use std::sync::Mutex;

use crate::{Files, general_helper};

pub fn load_icon_by_path(file_path: &str) -> Option<TaoIcon> {
    return if let Ok(image) = image::open(file_path) {
        let (width, height) = image.dimensions();
        let rgba_data = image.to_rgba8().into_raw();
        Some(TaoIcon::from_rgba(rgba_data, width, height).expect("Failed to load icon."))
    } else {
        None
    }
}

pub fn close_application(cx: Scope) {
    let window = dioxus_desktop::use_window(&cx);
    window.close_window(window.id());
}

pub fn validate_clicked_id_on_click(files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    let converted_clicked_directory_id: usize =
        general_helper::get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());

    if converted_clicked_directory_id != general_helper::get_converted_usize_from_string("0".to_string()) {
        return files.write().enter_directory(converted_clicked_directory_id);
    }
}

pub fn get_icon_type(path: String) -> String {
    return match std::fs::metadata(path.clone()) {
        Ok(metadata) => {
            if path.ends_with(".zip") {
                "folder_zip".to_string()
            } else if metadata.is_dir() {
                "folder".to_string()
            } else if metadata.is_file() {
                "description".to_string()
            } else {
                return "None".to_string();
            }
        }
        Err(error) => {
            return error.to_string();
        }
    }
}

pub fn clean_clicked_directory_id(clicked_directory_id: &Mutex<usize>) {
    let clean_up_value: usize = 0;
    *clicked_directory_id.lock().unwrap() = clean_up_value;
}
