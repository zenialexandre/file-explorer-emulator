use dioxus::prelude::*;
use crate::{Files, window_helper};
use crate::copy_and_paste_operation::COPIED_FILE_OR_DIR_NAME;

pub(crate) fn check_file_or_dir_conflict(mut selected_current_stack: String, files: &UseRef<Files>) -> bool {
    let copied_file_or_dir_name = COPIED_FILE_OR_DIR_NAME.lock().unwrap().last().unwrap().to_string();
    selected_current_stack.push_str(format!("\\{}", copied_file_or_dir_name).as_str());

    if files.read().path_names.contains(&selected_current_stack) {
        return true;
    }
    return false;
}

pub(crate) fn with_conflict_process(cx: Scope, files: &UseRef<Files>, selected_current_stack: String, copied_file_or_dir_name_joined: String) {
    let conflict_dom: VirtualDom = VirtualDom::new_with_props(conflict_popup, conflict_popupProps
    { files_props: files.clone(), copied_file_or_dir_props: copied_file_or_dir_name_joined, selected_current_stack_props: selected_current_stack });
    window_helper::create_new_dom_generic_window(cx, conflict_dom, "Conflict");
}

#[inline_props]
fn conflict_popup(cx: Scope, files_props: UseRef<Files>, copied_file_or_dir_props: String, selected_current_stack_props: String) -> Element {
    cx.render(rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            style { include_str!("./assets/conflict_popup.css") }
            div {
                class: "central-div",
                i { class: "material-icons", {}, "warning" }
                h1 { "Your operation generated a conflict!" }
            }
        }
    })
}
