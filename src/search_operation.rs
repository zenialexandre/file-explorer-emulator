use std::collections::HashMap;
use std::ops::Not;
use std::string::ToString;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;
use walkdir::WalkDir;

use crate::Files;
use crate::window_helper;
use crate::REGULAR_FILE;

pub(crate) fn create_search_input_field<'a>(cx: &'a ScopeState, files: &'a UseRef<Files>,
                                            is_search_field_enabled: &'a UseState<bool>) -> LazyNodes<'a, 'a> {
    let search_value: &UseState<String> = use_state(cx, || String::new());

    if is_search_field_enabled.get() == &true {
        let search_field_assets = r"
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
                oninput: |type_event| {
                    search_value.set(type_event.value.to_string());
                },
                onkeydown: |keydown_event| {
                    if keydown_event.inner().code() == Code::Enter {
                        execute_search_operation(cx, files, search_value.to_string().trim().to_string());
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

fn execute_search_operation(cx: &ScopeState, files: &UseRef<Files>, search_value: String) {
    let search_results_map = use_ref(cx, || HashMap::new());

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
    let mut iteration_counter = 1;

    for dir_entry in WalkDir::new(files.read().current()).into_iter().filter_map(|dir_entry_mapped| dir_entry_mapped.ok()) {
        let name = dir_entry.file_name().to_string_lossy().to_string();
        let path_name = dir_entry.path().to_string_lossy().to_string();
        if name.contains(search_value.as_str()) {
            search_results_map.write().insert(iteration_counter, path_name);
        }
        iteration_counter += 1;
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
            .with_title("Search").with_inner_size(LogicalSize::new(680.0, 400.0)))
    );
}

#[inline_props]
pub(crate) fn search_results_popup(cx: Scope, files_props: UseRef<Files>, search_results_map_props: UseRef<HashMap<usize, String>>) -> Element {
    let searched_object_path_clicked = use_state(cx, || "".to_string());

    cx.render(rsx!(
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { include_str!("./assets/search_popup.css") }
            header {
                i { class: "material-symbols-outlined", onclick: move |_| {
                   open_content_location(cx, files_props, searched_object_path_clicked.get().to_string());
                }, "folder_open" },
                h1 { top: "-7px;", "Open File/Folder Location." },
                span { }
                i { top: "-7px;", class: "material-icons", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                }, "cancel" }
            },
            div {
                main {
                    create_search_results_table(cx.scope, files_props, search_results_map_props.clone(), searched_object_path_clicked.clone())
                }
            }
        }
    ))
}

fn create_search_results_table<'a>(cx: &'a ScopeState, files_props: &'a UseRef<Files>,
                                   search_results_map_props: UseRef<HashMap<usize, String>>,
                                   searched_object_path_clicked: UseState<String>) -> LazyNodes<'a, 'a> {
    if search_results_map_props.read().is_empty() {
        rsx!(
            i { class: "material-symbols-outlined", {}, "sentiment_dissatisfied" }
            br {},
            h1 { "Nothing found." }
        )
    } else {
        rsx!(
            search_results_map_props.read().iter().map(|searched_object| {
                let searched_object_path = searched_object.1.to_string();
                let icon_type = window_helper::get_icon_type(searched_object_path.clone());

                rsx!(
                    table {
                        tbody {
                            tr {
                                class: "folder",
                                tabindex: "0",
                                onclick: move |_| {
                                    //searched_object_path_clicked.set(searched_object_path.clone());
                                },
                                ondblclick: move |_| {
                                    open_content(cx, files_props, searched_object_path.clone());
                                },
                                td { i { class: "material-icons", "{icon_type}" } },
                                td { h1 { "{searched_object_path}" } }
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
        let entry_stack_joined = entry_stack.join("\\");
        files_props.write().path_stack.push(entry_stack_joined);
        files_props.write().reload_path_list();
        dioxus_desktop::use_window(cx).close();
    }
}

fn open_content(cx: &ScopeState, files_props: &UseRef<Files>, searched_object_path: String) {
    let searched_object_path_internal = searched_object_path.clone();

    if window_helper::get_file_type_formatted(searched_object_path_internal.clone()) == REGULAR_FILE {
        window_helper::open_file(searched_object_path_internal.as_str());
    } else {
        files_props.write().path_stack.push(searched_object_path_internal.clone().to_string());
        files_props.write().reload_path_list();
        dioxus_desktop::use_window(cx).close();
    }
}
