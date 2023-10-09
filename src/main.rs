mod window_helper;
mod rename_operation;
mod delete_operation;
mod create_operation;
mod copy_and_paste_operation;
mod conflict_process;
mod cut_operation;
mod context_menu;
mod search_operation;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::html::input_data::keyboard_types::{Code, Modifiers};
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use dioxus_desktop::tao::dpi::LogicalPosition;

use crate::context_menu::{context_menu_popup, context_menu_popupProps};
use crate::delete_operation::{delete_popup, delete_popupProps};
use crate::create_operation::{create_rename_popup, create_rename_popupProps};
use crate::conflict_process::{conflict_popup, conflict_popupProps};
use crate::copy_and_paste_operation::COPY_INCREMENTAL_ID;
use crate::context_menu::IS_CONTEXT_ON_ITEM;

#[macro_use]
extern crate lazy_static;

lazy_static! { pub(crate) static ref CLICKED_DIRECTORY_ID: Mutex<usize> = Mutex::new(0); }
lazy_static! { pub(crate) static ref NEW_FILE_OR_DIR_NAME: Mutex<String> = Mutex::new("".to_string()); }
lazy_static! { pub(crate) static ref PREVIOUS_OPERATION_DONE: Mutex<String> = Mutex::new("".to_string()); }
lazy_static! { static ref MAIN_ASSETS: Mutex<String> = Mutex::new("".to_string()); }
lazy_static! { static ref FOLDER_ASSETS: Mutex<String> = Mutex::new("".to_string()); }

const REGULAR_FILE: &str = "Regular File";

#[derive(Clone)]
pub(crate) struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    error: Option<String>,
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_disable_context_menu(true).with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(LogicalSize::new(1100.0, 800.0))
            .with_position(LogicalPosition::new(100, 50))
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_focused(true)
        )
    );
}

fn app(cx: Scope) -> Element {
    let main_element: &UseRef<Vec<Event<MountedData>>> = use_ref(cx, || Vec::new());
    let files: &UseRef<Files> = use_ref(cx, Files::new);
    let context_menu_active: &UseState<bool> = use_state(cx, || false);
    let is_search_field_enabled: &UseState<bool> = use_state(cx, || false);
    let is_table_layout_triggered: &UseState<bool> = use_state(cx, || false);
    *MAIN_ASSETS.lock().unwrap() = "padding: 20px 60px;".to_string();
    *FOLDER_ASSETS.lock().unwrap() = r"
        float: left;
        width: 100px;
        height: 152px;
        /* //padding: 20px; */
        margin-right: 50px;
        margin-bottom: 70px;
        border-radius: 2px;
        /* //overflow: hidden; */
        cursor: pointer;
    ".to_string();

    cx.render(rsx! {
        div {
            id: "main-div",
            autofocus: "true",
            tabindex: "0",
            onmounted: move |element| {
                main_element.write().push(element);
            },
            onclick: move |click_event| {
                handle_click_event(cx, click_event, context_menu_active);
                window_helper::set_element_focus(main_element);
            },
            onkeydown: move |keydown_event| {
                handle_general_keyboard_events(cx, files, keydown_event, is_table_layout_triggered);
            },
            oncontextmenu: move |context_menu_event| {
                handle_context_menu_event(cx, files, context_menu_event, context_menu_active);
                *IS_CONTEXT_ON_ITEM.lock().unwrap() = false;
            },
            link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { include_str!("./assets/styles.css") }
            header {
                i { class: "material-icons", onclick: move |_| files.write().walk_to_last_directory(), "arrow_back" }
                i { class: "material-icons", onclick: move |_| window_helper::validate_clicked_id_on_click(files, &CLICKED_DIRECTORY_ID), "arrow_forward" }
                h1 { files.read().current() }
                span { }
                search_operation::create_search_input_field(cx, files, is_search_field_enabled)
                i { class: "material-symbols-outlined", onclick: move |_| {
                    context_menu::close_context_menu_on_demand(cx);
                    if is_search_field_enabled == &true {
                        is_search_field_enabled.set(false);
                    } else {
                        is_search_field_enabled.set(true);
                    }
                }, "search" }
                i { class: "material-icons", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                    context_menu::close_context_menu_on_demand(cx);
                }, "cancel" }
            },
            /*
            TODO -> Left div with the directories-tree.
            div {
                class: "left-panel",
                files.read().path_stack.iter().enumerate().map(|(directory_id, path)| {
                    rsx!(
                        div {
                            class: "directories-tree",
                            ondblclick: move |_| {

                            },
                            i { class: "material-symbols-outlined", {}, "keyboard_arrow_right" },
                            //i { class: "material-symbols-outlined", {}, "keyboard_arrow_down" }
                            h1 { "{path}" }
                        }
                    )
                })
            },
            div { class: "separator" },*/
            div {
                //class: "right-panel",
                main {
                    style: "{MAIN_ASSETS.lock().unwrap()}",
                    files.read().path_names.iter().enumerate().map(|(directory_id, path)| {
                        let path_end = path.split('\\').last().unwrap_or(path.as_str());
                        let icon_type: String = window_helper::get_icon_type(path.to_string());
                        let file_type: String = window_helper::get_file_type_formatted(path.to_string());
                        let path_metadata = std::fs::metadata(path.to_string());
                        let file_size: u64 = window_helper::get_file_size(path.to_string());
                        let mut last_modification_date_utc: DateTime<Utc> = Default::default();
                        let mut _last_modification_date_formatted: String = String::new();

                        if let Ok(path_metadata) = path_metadata.expect("Modified").modified() {
                            last_modification_date_utc = path_metadata.into();
                        }
                        _last_modification_date_formatted =
                            last_modification_date_utc.format("%d/%m/%Y %H:%M:%S").to_string().split('.').next().expect("Next").to_string();

                        rsx! (
                            div {
                                class: "folder",
                                style: "{FOLDER_ASSETS.lock().unwrap()}",
                                key: "{path}",
                                tabindex: "0",
                                onkeydown: move |keydown_event| {
                                    handle_main_keyboard_events(cx, files, keydown_event);
                                },
                                ondblclick: move |_| {
                                    handle_double_click_event(files, directory_id, main_element);
                                },
                                onclick: move |click_event| {
                                    handle_click_event(cx, click_event, context_menu_active);
                                    *CLICKED_DIRECTORY_ID.lock().unwrap() = directory_id;
                                },
                                oncontextmenu: move |context_menu_event| {
                                    handle_context_menu_event(cx, files, context_menu_event, context_menu_active);
                                    *CLICKED_DIRECTORY_ID.lock().unwrap() = directory_id;
                                    *IS_CONTEXT_ON_ITEM.lock().unwrap() = true;
                                },
                                set_layout_option(is_table_layout_triggered.get(), icon_type, path_end.to_string(),
                                    path.to_string(), _last_modification_date_formatted, file_type, file_size)
                            }
                        )
                    }),
                    files.read().error.as_ref().map(|err| {
                        rsx! (
                            div {
                                code { "{err}" }
                                button { onclick: move |_| files.write().clear_error(), "x" }
                            }
                        )
                    })
                }
            }
        }
    })
}

fn set_layout_option(is_table_layout_triggered: &bool, icon_type: String, path_end: String, path: String,
                     _last_modification_date_formatted: String, file_type: String, file_size: u64) -> LazyNodes {
    if is_table_layout_triggered == &true {
        activate_table_layout(icon_type, path_end, path, _last_modification_date_formatted, file_type, file_size)
    } else {
        activate_images_layout(icon_type, path_end, path, _last_modification_date_formatted, file_type, file_size)
    }
}

fn activate_table_layout<'a>(icon_type: String, path_end: String, path: String, _last_modification_date_formatted: String,
                             file_type: String, file_size: u64) -> LazyNodes<'a, 'a> {
    *MAIN_ASSETS.lock().unwrap() = r"
        padding: 20px 60px;
        display: flex;
        flex-direction: column;
        outline: none;
        height: 0px;
    ".to_string();

    *FOLDER_ASSETS.lock().unwrap() = r"
        float: left;
        width: 100px;
        height: 0px;
        /* //padding: 20px; */
        margin-right: 50px;
        margin-bottom: 70px;
        border-radius: 2px;
        /* //overflow: hidden; */
        cursor: pointer;
    ".to_string();

    let table_assets = r"
        height: auto;
        width: auto;
        padding: 2px;
        border-collapse: separate;
        border-spacing: 2px;
        border: 1px solid gray;
        border-radius: 5px;
        margin-top: 20px;
        line-height: 1;
    ";

    let i_assets = r"
        text-align: left;
        font-size: 20px;
        color: #607D8B;
    ";

    let h1_assets = r"
        top: -7px;
        font-size: 15px;
        width: 250px;
        font-weight: 4px;
        text-align: left;
        padding-left: 50px;
        color: black;
        word-wrap: break-word;
    ";

    rsx!(
        table {
            style: "{table_assets}",
            tbody {
                tr {
                    td { i { style: "{i_assets}", class: "material-icons", "{icon_type}" } },
                    td { h1 { style: "{h1_assets}", "{path_end}" } },
                    td { h1 { style: "{h1_assets}", "{_last_modification_date_formatted}" } },
                    td { h1 { style: "{h1_assets}", "{file_type}" } },
                    if window_helper::get_file_type_formatted(path.to_string()) == REGULAR_FILE.to_string() {
                        rsx!( td { h1 { style: "{h1_assets}", "{file_size} KB" } } )
                    } else {
                        rsx!( td { h1 { style: "{h1_assets}", "N/A" } } )
                    }
                }
            }
        }
    )
}

fn activate_images_layout<'a>(icon_type: String, path_end: String, path: String, _last_modification_date_formatted: String,
                              file_type: String, file_size: u64) -> LazyNodes<'a, 'a> {
    let i_assets = r"
        margin: 0;
        font-size: 80px;
        color: #607D8B;
    ";

    let h1_assets = r"
        position: relative;
        display: block;
        top: -5px;
        font-size: 13px;
        font-weight: 12px;
        text-align: center;
        padding-right: -7px;
        word-wrap: break-word;
    ";

    rsx!(
        i { style: "{i_assets}", class: "material-icons", "{icon_type}" },
        h1 { style: "{h1_assets}", "{path_end}" },
        p { class: "cooltip", "{_last_modification_date_formatted}" },
        p { class: "cooltip", "{file_type}" },
        if window_helper::get_file_type_formatted(path.to_string()) == REGULAR_FILE.to_string() {
            rsx!( p { class: "cooltip", "{file_size} KB" } )
        }
    )
}

fn handle_general_keyboard_events(cx: Scope, files: &UseRef<Files>, keydown_event: Event<KeyboardData>, is_table_layout_triggered: &UseState<bool>) {
    if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyN {
        let create_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup,
            create_rename_popupProps { files_props: files.clone(), title_props: "Create" });
        window_helper::create_new_dom_generic_window(cx, create_dom, "Create");
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyV {
        copy_and_paste_operation::execute_paste_operation(files, &PREVIOUS_OPERATION_DONE);
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) &&
        (keydown_event.inner().code() == Code::Equal || keydown_event.inner().code() == Code::NumpadAdd) {
        is_table_layout_triggered.set(false);
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) &&
        (keydown_event.inner().code() == Code::Minus || keydown_event.inner().code() == Code::NumpadSubtract) {
        is_table_layout_triggered.set(true);
    }
}

fn handle_main_keyboard_events(cx: Scope, files: &UseRef<Files>, keydown_event: Event<KeyboardData>) {
    if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyR {
        let rename_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup,
            create_rename_popupProps { files_props: files.clone(), title_props: "Rename" });
        window_helper::create_new_dom_generic_window(cx, rename_dom, "Rename");
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyD {
        let delete_dom: VirtualDom = VirtualDom::new_with_props(delete_popup, delete_popupProps { files_props: files.clone() });
        window_helper::create_new_dom_generic_window(cx, delete_dom, "Delete");
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyC {
        copy_and_paste_operation::execute_copy_operation(files, &CLICKED_DIRECTORY_ID);
    } else if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyX {
        cut_operation::execute_cut_operation(files, &CLICKED_DIRECTORY_ID);
    }
}

fn handle_double_click_event(files: &UseRef<Files>, directory_id: usize, main_element: &UseRef<Vec<Event<MountedData>>>) {
    let selected_full_path = window_helper::get_selected_full_path(files, &CLICKED_DIRECTORY_ID);
    match std::fs::metadata(selected_full_path.clone()) {
        Ok(path_metadata) => {
            if path_metadata.is_file() {
                window_helper::open_file(selected_full_path.clone().as_str());
            } else if path_metadata.is_dir() {
                files.write().enter_directory(directory_id);
                window_helper::set_element_focus(main_element);
            }
        },
        Err(error) => panic!("{}", error)
    }
}

fn handle_click_event(cx: Scope, click_event: Event<MouseData>, context_menu_active: &UseState<bool>) {
    click_event.stop_propagation();
    context_menu::close_context_menu(cx, context_menu_active);
    context_menu_active.set(false);
}

fn handle_context_menu_event(cx: Scope, files: &UseRef<Files>, context_menu_event: Event<MouseData>, context_menu_active: &UseState<bool>) {
    context_menu_event.stop_propagation();
    context_menu::close_context_menu_on_demand(cx);
    context_menu_active.set(true);

    let context_menu_dom: VirtualDom = VirtualDom::new_with_props(context_menu_popup,
        context_menu_popupProps { files_props: files.clone() });
    context_menu::create_context_menu(cx, context_menu_dom, context_menu_event.client_coordinates());
}

impl Files {
    fn new() -> Self {
      let mut files = Self {
          path_stack: vec!["C://".to_string()],
          path_names: vec![],
          error: None,
      };
      files.reload_path_list();
      files
    }

    fn reload_path_list(&mut self) {
        let current_path = self.path_stack.last().unwrap();
        let paths = match std::fs::read_dir(current_path) {
            Ok(e) => e,
            Err(error) => {
                let error = format!("An error occurred: {error:?}");
                self.error = Some(error);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();

        self.clear_error();
        self.path_names.clear();

        for path in collected {
            self.path_names.push(path.unwrap().path().display().to_string());
        }
    }

    fn walk_to_last_directory(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        window_helper::clean_lazy_static_value(&CLICKED_DIRECTORY_ID,  &COPY_INCREMENTAL_ID);
        self.reload_path_list();
    }

    fn enter_directory(&mut self, directory_id: usize) {
        if let Some(_path_name) = &self.path_names.get(directory_id) {
            let path = &self.path_names[directory_id];
            self.path_stack.push(path.clone());
            window_helper::clean_lazy_static_value(&CLICKED_DIRECTORY_ID, &COPY_INCREMENTAL_ID);
            self.reload_path_list();
        }
    }

    fn current(&self) -> &str {
        return self.path_stack.last().unwrap();
    }

    fn clear_error(&mut self) {
        self.error = None;
    }

}
