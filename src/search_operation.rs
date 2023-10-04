use std::collections::HashMap;
use std::string::ToString;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
//use regex::Regex;
use walkdir::WalkDir;

use crate::Files;
use crate::window_helper;

pub(crate) fn create_search_input_field<'a>(cx: &'a ScopeState, files: &'a UseRef<Files>,
                                            is_search_field_enabled: &'a UseState<bool>) -> LazyNodes<'a, 'a> {
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
        let search_value: &UseState<String> = use_state(cx, || String::new());

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
    if search_value.is_empty() {
        files.write().path_stack.clear();
        files.write().path_names.clear();
        files.write().path_stack.push("C://".to_string());
        files.write().reload_path_list();
    } else {
        let search_results_map = use_ref(cx,  || HashMap::new());
        search_results_map.write().clear();
        search(files.clone(), search_results_map.clone(), search_value.clone());
        create_search_popup(cx, files.clone(), search_results_map.clone());
    }
}

fn search(files: UseRef<Files>, search_results_map: UseRef<HashMap<usize, String>>, search_value: String) {
    //let search_regex_pattern = search_value_props.chars().map(|char| format!(".*{}.*", char)).collect::<String>();
    //let search_regex = Regex::new(search_value_props.as_str()).unwrap();
    let mut iteration_counter = 1;

    for path_name in files.read().path_names.iter() {
        for entry in WalkDir::new(path_name).into_iter().filter_map(|entry_mapped| entry_mapped.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            let path_name = entry.path().to_string_lossy().to_string();
            if name.contains(search_value.as_str()) {
                search_results_map.write().insert(iteration_counter, path_name);
            }
            iteration_counter += 1;
        }
    }
}

fn create_search_popup(cx: &ScopeState, files: UseRef<Files>, search_results_map: UseRef<HashMap<usize, String>>) {
    let search_results_dom: VirtualDom = VirtualDom::new_with_props(search_results_popup,
                                                                    search_results_popupProps { files_props: files.clone(), search_result_map_props: search_results_map.clone() });
    window_helper::create_new_dom_generic_window_state(cx, search_results_dom, "Search");
}

#[inline_props]
pub(crate) fn search_results_popup(cx: Scope, files_props: UseRef<Files>, search_result_map_props: UseRef<HashMap<usize, String>>) -> Element {
    cx.render(rsx!(
        div {
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { include_str!("./assets/search_popup.css") }
            header {
                span { }
                i { class: "material-icons", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                }, "cancel" }
            },
            div {
                main {
                    table {
                        tbody {
                            search_result_map_props.read().iter().map(|searched_object| {
                                let path_borrowed = searched_object.1.to_string();

                                rsx!(
                                    tr {
                                        class: "folder",
                                        tabindex: "0",
                                        ondblclick: move |_| {
                                            println!("{}", path_borrowed);
                                        },
                                        td { h1 { "{path_borrowed}" } }
                                    }
                                )
                            })
                        }
                    }
                }
            }
        }
    ))
}

