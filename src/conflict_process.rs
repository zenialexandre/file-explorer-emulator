use dioxus::prelude::*;

use crate::Files;
use crate::create_operation;
use crate::{NEW_FILE_OR_DIR_NAME, GENERIC_POPUP_ID};

pub(crate) fn check_file_or_dir_conflict(file_or_dir_name: String, mut selected_current_stack: String, files: Signal<Files>) -> bool {
    selected_current_stack.push_str(format!("\\{}", file_or_dir_name).as_str());

    if files.read().path_names.contains(&selected_current_stack) {
        return true;
    }
    return false;
}

#[inline_props]
pub(crate) fn conflict_popup(files_props: Signal<Files>, enable_file_creation_props: Signal<bool>) -> Element {
    GENERIC_POPUP_ID.lock().unwrap().push(dioxus::desktop::use_window().id());
    let mut enable_rename_field: Signal<bool> = use_signal(|| false);

    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            style { { include_str!("./assets/conflict_popup.css") } }
            div {
                class: "central-div",
                i { class: "material-icons", {}, "warning" }
                h1 { "Your operation generated a conflict!" }
                br {}
                label {
                    class: "central-div-label",
                    i {
                        class: "material-icons",
                        onclick: move |_| {
                            dioxus::desktop::use_window().close();
                        },
                        "cancel"
                    },
                    h1 { "Cancel the operation." }
                },
                label {
                    class: "central-div-label",
                    input {
                        r#type: "checkbox",
                        checked: "{enable_rename_field}",
                        id: "enable_rename_field",
                        oninput: move |check_event: Event<FormData>| {
                            enable_rename_field.set(check_event.value().parse().unwrap());
                        }
                    }
                    h1 { "Check if you wish to rename." }
                },

                if enable_rename_field.read().clone() == true {
                    {
                        rsx!(
                            div {
                                class: "central-div-label",
                                br {},
                                input {
                                    autofocus: "true",
                                    r#type: "text",
                                    placeholder: "Directory/File new name",
                                    id: "directory-file-name",
                                    oninput: |type_event: Event<FormData>| {
                                        *NEW_FILE_OR_DIR_NAME.lock().unwrap() = type_event.value().to_string()
                                    }
                                },
                                i {
                                    class: "material-icons",
                                    onclick: move |_| {
                                        create_operation::execute_create_operation(files_props, &NEW_FILE_OR_DIR_NAME, &enable_file_creation_props);
                                        dioxus::desktop::use_window().close();
                                    },
                                    "check_circle"
                                }
                            }
                        )
                    }
                }
            }
        }
    }
}
