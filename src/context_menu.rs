use std::ops::{Deref, Not};
use dioxus::prelude::*;
use dioxus::desktop::{
    Config,
    WindowBuilder,
    tao::{
        dpi::{
            PhysicalPosition,
            LogicalSize
        },
        window::WindowId,
        platform::windows::WindowBuilderExtWindows
    }
};
use std::sync::Mutex;
use dioxus::html::geometry::ClientPoint;

use crate::Files;
use crate::{window_helper, copy_and_paste_operation, cut_operation};
use crate::delete_operation::{delete_popup, delete_popupProps};
use crate::change_root_path_operation::{change_root_path_popup, change_root_path_popupProps};
use crate::create_operation::{create_rename_popup, create_rename_popupProps};
use crate::{CLICKED_DIRECTORY_ID, PREVIOUS_OPERATION_DONE};

lazy_static! { pub(crate) static ref CONTEXT_MENU_ID: Mutex<Vec<WindowId>> = Mutex::new(Vec::new()); }
lazy_static! { pub(crate) static ref IS_CONTEXT_ON_ITEM: Mutex<bool> = Mutex::new(false); }

pub(crate) fn create_context_menu(context_menu_dom: VirtualDom, context_menu_position: ClientPoint) {
    dioxus::desktop::use_window().new_window(context_menu_dom, Config::default()
        .with_window(WindowBuilder::new()
            .with_position(PhysicalPosition::new(context_menu_position.x, context_menu_position.y))
            .with_resizable(false)
            .with_focused(true)
            .with_closable(false)
            .with_drag_and_drop(false)
            .with_skip_taskbar(false)
            .with_title("")
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_inner_size(LogicalSize::new(300.0, 430.0))
        )
    );
}

pub(crate) fn close_context_menu(context_menu_active: &Signal<bool>) {
    if (context_menu_active.read().clone()).not() && CONTEXT_MENU_ID.lock().unwrap().len() > 0 {
        dioxus::desktop::use_window().close_window(CONTEXT_MENU_ID.lock().unwrap().pop().unwrap());
    }
}

pub(crate) fn close_context_menu_on_demand() {
    if CONTEXT_MENU_ID.lock().unwrap().len() > 0 {
        dioxus::desktop::use_window().close_window(CONTEXT_MENU_ID.lock().unwrap().pop().unwrap());
    }
}

#[inline_props]
pub(crate) fn context_menu_popup(files_props: Signal<Files>) -> Element {
    CONTEXT_MENU_ID.lock().unwrap().push(dioxus::desktop::use_window().id());

    rsx! {
        div {
            autofocus: "true",
            tabindex: "0",
            onmounted: move |mounted_event: Event<MountedData>| async move {
                let _ = mounted_event.set_focus(true).await;
            },
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { { include_str!("./assets/context_menu_popup.css") } }
            div {
                class: "context-menu",
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus::desktop::use_window().close();
                    let create_dom: VirtualDom = VirtualDom::new_with_props(
                        create_rename_popup,
                        create_rename_popupProps { files_props: files_props.clone(), title_props: "Create".to_string() }
                    );
                    window_helper::create_new_dom_generic_window_state(create_dom, "Create");
                }, label { i { class: "material-icons", "folder" }, "New / Ctrl+N" } },
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus::desktop::use_window().close();
                    copy_and_paste_operation::execute_paste_operation(files_props, &PREVIOUS_OPERATION_DONE);
                }, label { i { class: "material-icons", "content_paste" }, "Paste / Ctrl+V" } },
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus::desktop::use_window().close();
                    let change_root_path_dom: VirtualDom = VirtualDom::new_with_props(
                        change_root_path_popup,
                        change_root_path_popupProps { files_props: files_props.clone() }
                    );
                    window_helper::create_new_dom_generic_window_state(change_root_path_dom, "Change Root Path");
                }, label { i { class: "material-symbols-outlined", "home_storage" }, "Change Root Path / Ctrl+B" } },

                if IS_CONTEXT_ON_ITEM.lock().unwrap().deref() == &true {
                        {rsx!(
                            div { class: "context-menu-item", onclick: move |_| {
                                dioxus::desktop::use_window().close();
                                copy_and_paste_operation::execute_copy_operation(files_props, &CLICKED_DIRECTORY_ID);
                            }, label { i { class: "material-icons", "content_copy" }, "Copy / Ctrl+C" } },
                            div { class: "context-menu-item", onclick: move |_| {
                                dioxus::desktop::use_window().close();
                                cut_operation::execute_cut_operation(files_props, &CLICKED_DIRECTORY_ID);
                            }, label { i { class: "material-icons" , "content_cut" }, "Cut / Ctrl+X" } },
                            div { class: "context-menu-item",  onclick: move |_| {
                                dioxus::desktop::use_window().close();
                                let rename_dom: VirtualDom = VirtualDom::new_with_props(
                                    create_rename_popup,
                                    create_rename_popupProps { files_props: files_props.clone(), title_props: "Rename".to_string() }
                                );
                                window_helper::create_new_dom_generic_window_state(rename_dom, "Rename");
                            }, label { i { class: "material-icons", "edit" }, "Rename / Ctrl+R" } },
                            div { class: "context-menu-item", onclick: move |_| {
                                dioxus::desktop::use_window().close();
                                let delete_dom: VirtualDom = VirtualDom::new_with_props(
                                    delete_popup,
                                    delete_popupProps { files_props: files_props.clone() }
                                );
                                window_helper::create_new_dom_generic_window_state(delete_dom, "Delete");
                            }, label { i { class: "material-icons", "delete" }, "Delete / Ctrl+D" } },
                        )
                    }
                }
            }
        },
    }
}
