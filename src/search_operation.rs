use std::collections::HashMap;
use std::ops::Not;
use std::rc::Rc;
use std::string::ToString;
use std::sync::Mutex;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::dpi::LogicalPosition;
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;
use walkdir::WalkDir;

use crate::Files;
use crate::window_helper;
use crate::{REGULAR_FILE, GENERIC_POPUP_ID};

lazy_static! { static ref SEARCHED_PATH_CLICKED: Mutex<String> = Mutex::new("".to_string()); }

pub(crate) fn create_search_input_field<'a>(cx: &'a ScopeState, files: &'a UseRef<Files>,
                                            is_search_field_enabled: &'a UseState<bool>) -> LazyNodes<'a, 'a> {
    let search_value: &UseState<String> = use_state(cx, || String::new());
    let search_results_map: &UseRef<HashMap<usize, String>> = use_ref(cx, || HashMap::new());

    if is_search_field_enabled.get() == &true {
        let search_field_assets: &str = r"
            text-align: left;
            font-size: 13px;
            font-weight: 2px;
            font-family: 'Rubik', sans-serif;
            border-radius: 5px;
            width: 280px;
            height: 18px;
            padding-left: 3px;
        ";

        rsx!(
            input {
                id: "search-field",
                style: "{search_field_assets}",
                autofocus: "true",
                r#type: "text",
                placeholder: "Search inside the current stack...",
                oninput: |type_event: Event<FormData>| {
                    search_value.set(type_event.value.to_string());
                },
                onkeydown: |keydown_event: Event<KeyboardData>| {
                    if keydown_event.inner().code() == Code::Enter {
                        execute_search_operation(cx, files, search_results_map, search_value.to_string().trim().to_string());
                    }
                }
            },
        )
    } else {
        rsx!(
            p {}
        )
    }
}

fn execute_search_operation(cx: &ScopeState, files: &UseRef<Files>, search_results_map: &UseRef<HashMap<usize, String>>, search_value: String) {
    if search_value.is_empty() {
        files.write().path_stack.clear();
        files.write().path_names.clear();
        files.write().path_stack.push("C://".to_string());
        files.write().reload_path_list();
    } else {
        search_results_map.write().clear();
        search(files.clone(), search_results_map.clone(), search_value.clone());
        create_search_popup(cx, files.clone(), search_results_map.clone());
    }
}

fn search(files: UseRef<Files>, search_results_map: UseRef<HashMap<usize, String>>, search_value: String) {
    let directories_filtered: Vec<String> = WalkDir::new(files.read().current())
        .into_iter().filter_map(|dir_entry| dir_entry.ok())
        .filter(|dir_entry| dir_entry.file_name().to_string_lossy().contains(&search_value))
        .map(|dir_entry| dir_entry.path().to_string_lossy().to_string()).collect();

    for (iteration_counter, path_name) in directories_filtered.iter().enumerate() {
        search_results_map.write().insert(iteration_counter + 1, path_name.clone());
    }
}

fn create_search_popup(cx: &ScopeState, files: UseRef<Files>, search_results_map: UseRef<HashMap<usize, String>>) {
    let search_results_dom: VirtualDom = VirtualDom::new_with_props(search_results_popup,
        search_results_popupProps { files_props: files, search_results_map_props: search_results_map.clone() });
    dioxus_desktop::use_window(cx).new_window(search_results_dom, Config::default()
        .with_window(WindowBuilder::new()
            .with_resizable(true).with_focused(true)
            .with_closable(false).with_drag_and_drop(false).with_skip_taskbar(false)
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_position(LogicalPosition::new(100, 50))
            .with_title("Search").with_inner_size(LogicalSize::new(680.0, 450.0)))
    );
}

#[inline_props]
pub(crate) fn search_results_popup(cx: Scope, files_props: UseRef<Files>, search_results_map_props: UseRef<HashMap<usize, String>>) -> Element {
    GENERIC_POPUP_ID.lock().unwrap().push(dioxus_desktop::use_window(cx).id());

    cx.render(rsx!(
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { include_str!("./assets/search_popup.css") }
            header {
                i { class: "material-symbols-outlined", onclick: move |_| {
                   open_content_location(cx, files_props, SEARCHED_PATH_CLICKED.lock().unwrap().to_string());
                }, "folder_open" },
                h1 { id: "open-content-h1", "Open Location." },
                span { }
                i { class: "material-icons", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                }, "cancel" }
            },
            div {
                main {
                    create_search_results_table(cx.scope, files_props, search_results_map_props.clone())
                }
            }
        }
    ))
}

fn create_search_results_table<'a>(cx: &'a ScopeState, files_props: &'a UseRef<Files>,
                                   search_results_map_props: UseRef<HashMap<usize, String>>) -> LazyNodes<'a, 'a> {
    if search_results_map_props.read().is_empty() {
        rsx!(
            i { class: "material-symbols-outlined", {}, "sentiment_dissatisfied" }
            br {},
            h1 { "Nothing found." }
        )
    } else {
        rsx!(
            search_results_map_props.read().iter().map(|searched_object| {
                let searched_path: Rc<String> = Rc::new(searched_object.1.to_string());
                let icon_type: String = window_helper::get_icon_type(searched_path.to_string());
                let searched_path_on_click: Rc<String> = Rc::clone(&searched_path);
                let searched_path_on_dbclick: Rc<String> = Rc::clone(&searched_path);

                rsx!(
                    table {
                        tbody {
                            tr {
                                class: "folder",
                                tabindex: "0",
                                onclick: move |_| {
                                    *SEARCHED_PATH_CLICKED.lock().unwrap() = searched_path_on_click.to_string();
                                },
                                ondblclick: move |_| {
                                    open_content(cx, files_props, searched_path_on_dbclick.to_string());
                                },
                                td { i { class: "material-icons", "{icon_type}" } },
                                td { h1 { "{searched_path}" } }
                            }
                        }
                    }
                )
            })
        )
    }
}

fn open_content_location(cx: &ScopeState, files_props: &UseRef<Files>, searched_object_path: String) {
    if searched_object_path.is_empty().not() {
        let mut entry_stack: Vec<&str> = searched_object_path.split("\\").collect();
        entry_stack.pop();
        let entry_stack_joined: String = entry_stack.join("\\");
        window_helper::open_folder(cx, files_props, entry_stack_joined.clone());
    }
}

fn open_content(cx: &ScopeState, files_props: &UseRef<Files>, searched_object_path: String) {
    if window_helper::get_file_type_formatted(searched_object_path.clone()) == REGULAR_FILE {
        window_helper::open_file(searched_object_path.as_str());
    } else {
        window_helper::open_folder(cx, files_props, searched_object_path);
    }
}
