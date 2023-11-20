use std::ops::Not;
use dioxus::prelude::*;
use dioxus_desktop::tao::window::{Icon as TaoIcon, WindowId};
use image::GenericImageView;
use std::sync::Mutex;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;

use crate::Files;

pub(crate) fn load_icon_by_path(file_path: &str) -> Option<TaoIcon> {
    return if let Ok(image) = image::open(file_path) {
        let (width, height) = image.dimensions();
        let rgba_data: Vec<u8> = image.to_rgba8().into_raw();
        Some(TaoIcon::from_rgba(rgba_data, width, height).expect("Failed to load icon."))
    } else {
        None
    }
}

pub(crate) fn validate_clicked_id_on_click(files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    let converted_clicked_directory_id: usize = get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());

    if converted_clicked_directory_id >= get_converted_usize_from_string("0".to_string()) {
        return files.write().enter_directory(converted_clicked_directory_id);
    }
}

pub(crate) fn get_icon_type(path: String) -> String {
    return match std::fs::metadata(path.clone()) {
        Ok(metadata) => {
            if path.ends_with(".zip") {
                "folder_zip".to_string()
            } else if metadata.is_dir() {
                "folder".to_string()
            } else if metadata.is_file() {
                "description".to_string()
            } else {
                "None".to_string()
            }
        }
        Err(error) => {
            println!("{}", error);
            "".to_string()
        }
    }
}

pub(crate) fn get_file_type_formatted(path: String) -> String {
    return match std::fs::metadata(path.clone()) {
        Ok(metadata) => {
            if metadata.is_dir() {
                "File Folder".to_string()
            } else if metadata.is_file() {
                "Regular File".to_string()
            } else if metadata.is_symlink() {
                "Symlink File".to_string()
            } else {
                "None".to_string()
            }
        }
        Err(error) => {
            println!("{}", error);
            "".to_string()
        }
    }
}

pub(crate) fn get_file_size(path: String) -> u64 {
    return match std::fs::metadata(path.clone()) {
        Ok(metadata) => {
            (metadata.len() as f64 / 1000.00).ceil() as u64
        },
        Err(error) => {
            println!("{}", error);
            0
        }
    };
}

pub(crate) fn clean_lazy_static_value(clicked_directory_id: &Mutex<usize>, copy_incremental_id: &Mutex<u32>) {
    *clicked_directory_id.lock().unwrap() = "0".parse().unwrap();
    *copy_incremental_id.lock().unwrap() = 0;
}

pub(crate) fn get_converted_usize_from_string(any_string: String) -> usize {
    return any_string.parse().unwrap();
}

pub(crate) fn create_new_dom_generic_window(cx: Scope, generic_dom: VirtualDom, generic_window_name: &str) {
    dioxus_desktop::use_window(cx).new_window(generic_dom, Config::default()
        .with_window(WindowBuilder::new()
            .with_resizable(false).with_focused(true)
            .with_closable(false).with_drag_and_drop(false).with_skip_taskbar(false)
            .with_window_icon(load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_title(generic_window_name).with_inner_size(LogicalSize::new(700.0, 300.0)))
    );
}

pub(crate) fn create_new_dom_generic_window_state(cx: &ScopeState, generic_dom: VirtualDom, generic_window_name: &str) {
    dioxus_desktop::use_window(cx).new_window(generic_dom, Config::default()
        .with_window(WindowBuilder::new()
            .with_resizable(false).with_focused(true)
            .with_closable(false).with_drag_and_drop(false).with_skip_taskbar(false)
            .with_window_icon(load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_title(generic_window_name).with_inner_size(LogicalSize::new(700.0, 300.0)))
    );
}

pub(crate) fn get_selected_full_path(files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) -> String {
    let converted_clicked_directory_id: usize = get_converted_usize_from_string(clicked_directory_id.lock().unwrap().to_string());
    let selected_full_path: String = files.read().path_names[converted_clicked_directory_id].to_string();
    selected_full_path
}

pub(crate) fn get_selected_current_stack(files: &UseRef<Files>) -> String {
    let selected_current_stack: String = files.read().path_stack[files.read().path_stack.len() - 1].to_string();
    selected_current_stack
}

pub(crate) fn open_file(selected_path: &str) {
    opener::open(selected_path).unwrap_or_else(|error| println!("{}", error));
}

pub(crate) fn open_folder(cx: &ScopeState, files_props: &UseRef<Files>, searched_object_path: String) {
    let searched_object_path_splitted = searched_object_path.split("\\");
    files_props.write().path_stack.clear();

    for (index, splitted_path) in searched_object_path_splitted.enumerate() {
        if index == 0 {
            let home_path: Vec<&str> = splitted_path.split("//").collect();
            files_props.write().path_stack.push(format!("{}//", home_path.get(0).unwrap().to_string()));
            files_props.write().path_stack
                .push(format!("{}//{}", home_path.get(0).unwrap(), home_path.get(1).unwrap()));
        } else {
            let last_stack: String = files_props.read().path_stack.last().unwrap().to_string();
            files_props.write().path_stack
                .push(format!("{}\\{}", last_stack, splitted_path));
        }
    }
    files_props.write().reload_path_list();
    dioxus_desktop::use_window(cx).close();
}

pub(crate) fn set_element_focus(main_element: &UseRef<Vec<Event<MountedData>>>) {
    if let Some(element) = main_element.read().last() {
        element.set_focus(true);
    }
}

pub(crate) fn close_generic_popup_window(cx: Scope, mut generic_popup_id: Vec<WindowId>) {
    if generic_popup_id.is_empty().not() {
        dioxus_desktop::use_window(cx).close_window(generic_popup_id.pop().unwrap());
    }
}
