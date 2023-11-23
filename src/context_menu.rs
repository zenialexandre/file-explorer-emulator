use std::ops::{Deref, Not};
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_desktop::tao::platform::windows::WindowBuilderExtWindows;
use std::sync::Mutex;
use dioxus::html::geometry::ClientPoint;
use dioxus_desktop::tao::dpi::PhysicalPosition;
use dioxus_desktop::tao::window::WindowId;

use crate::Files;
use crate::{window_helper, copy_and_paste_operation, cut_operation};
use crate::delete_operation::{delete_popup, delete_popupProps};
use crate::change_root_path_operation::{change_root_path_popup, change_root_path_popupProps};
use crate::create_operation::{create_rename_popup, create_rename_popupProps};
use crate::{CLICKED_DIRECTORY_ID, PREVIOUS_OPERATION_DONE};

lazy_static! { pub(crate) static ref CONTEXT_MENU_ID: Mutex<Vec<WindowId>> = Mutex::new(Vec::new()); }
lazy_static! { pub(crate) static ref IS_CONTEXT_ON_ITEM: Mutex<bool> = Mutex::new(false); }

pub(crate) fn create_context_menu(cx: Scope, context_menu_dom: VirtualDom, context_menu_position: ClientPoint) {
    dioxus_desktop::use_window(cx).new_window(context_menu_dom, Config::default()
        .with_window(WindowBuilder::new().with_position(PhysicalPosition::new(context_menu_position.x, context_menu_position.y))
            .with_resizable(false).with_focused(true)
            .with_closable(false).with_drag_and_drop(false).with_skip_taskbar(false).with_title("")
            .with_window_icon(window_helper::load_icon_by_path("src/images/icon/cool_circle.png"))
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(300.0, 430.0)))
    );
}

pub(crate) fn close_context_menu(cx: Scope, context_menu_active: &UseState<bool>) {
    if (context_menu_active.get()).not() && CONTEXT_MENU_ID.lock().unwrap().len() > 0 {
        dioxus_desktop::use_window(cx).close_window(CONTEXT_MENU_ID.lock().unwrap().pop().unwrap());
    }
}

pub(crate) fn close_context_menu_on_demand(cx: Scope) {
    if CONTEXT_MENU_ID.lock().unwrap().len() > 0 {
        dioxus_desktop::use_window(cx).close_window(CONTEXT_MENU_ID.lock().unwrap().pop().unwrap());
    }
}

#[inline_props]
pub(crate) fn context_menu_popup(cx: Scope, files_props: UseRef<Files>) -> Element {
    CONTEXT_MENU_ID.lock().unwrap().push(dioxus_desktop::use_window(cx).id());

    cx.render(rsx! {
        div {
            autofocus: "true",
            tabindex: "0",
            onmounted: move |mounted_event: Event<MountedData>| {
                mounted_event.set_focus(true);
            },
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined", rel: "stylesheet", }
            style { include_str!("./assets/context_menu_popup.css") }
            div {
                class: "context-menu",
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                    let create_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup,
                        create_rename_popupProps { files_props: files_props.clone(), title_props: "Create" });
                    window_helper::create_new_dom_generic_window_state(cx, create_dom, "Create");
                }, label { i { class: "material-icons", "folder" }, "New / Ctrl+N" } },
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                    copy_and_paste_operation::execute_paste_operation(files_props, &PREVIOUS_OPERATION_DONE);
                }, label { i { class: "material-icons", "content_paste" }, "Paste / Ctrl+V" } },
                div { class: "context-menu-item", onclick: move |_| {
                    dioxus_desktop::use_window(cx).close();
                    let change_root_path_dom: VirtualDom = VirtualDom::new_with_props(change_root_path_popup, change_root_path_popupProps { files_props: files_props.clone() });
                    window_helper::create_new_dom_generic_window_state(cx.scope, change_root_path_dom, "Change Root Path");
                }, label { i { class: "material-symbols-outlined", "home_storage" }, "Change Root Path / Ctrl+B" } },

                if IS_CONTEXT_ON_ITEM.lock().unwrap().deref() == &true {
                    rsx!(
                        div { class: "context-menu-item", onclick: move |_| {
                            dioxus_desktop::use_window(cx).close();
                            copy_and_paste_operation::execute_copy_operation(files_props, &CLICKED_DIRECTORY_ID);
                        }, label { i { class: "material-icons", "content_copy" }, "Copy / Ctrl+C" } },
                        div { class: "context-menu-item", onclick: move |_| {
                            dioxus_desktop::use_window(cx).close();
                            cut_operation::execute_cut_operation(files_props, &CLICKED_DIRECTORY_ID);
                        }, label { i { class: "material-icons" , "content_cut" }, "Cut / Ctrl+X" } },
                        div { class: "context-menu-item",  onclick: move |_| {
                            dioxus_desktop::use_window(cx).close();
                            let rename_dom: VirtualDom = VirtualDom::new_with_props(create_rename_popup,
                                create_rename_popupProps { files_props: files_props.clone(), title_props: "Rename" });
                            window_helper::create_new_dom_generic_window_state(cx.scope, rename_dom, "Rename");
                        }, label { i { class: "material-icons", "edit" }, "Rename / Ctrl+R" } },
                        div { class: "context-menu-item", onclick: move |_| {
                            dioxus_desktop::use_window(cx).close();
                            let delete_dom: VirtualDom = VirtualDom::new_with_props(delete_popup, delete_popupProps { files_props: files_props.clone() });
                            window_helper::create_new_dom_generic_window_state(cx.scope, delete_dom, "Delete");
                        }, label { i { class: "material-icons", "delete" }, "Delete / Ctrl+D" } },
                    )
                }
            }
        },
    })
}
