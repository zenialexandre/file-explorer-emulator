use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;

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
                        execute_search_operation(cx, files, search_value);
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

fn execute_search_operation(cx: &ScopeState, files: &UseRef<Files>, search_value: &UseState<String>) {
    if search_value.get().to_string().trim().is_empty() {
        files.write().path_stack.clear();
        files.write().path_names.clear();
        files.write().path_stack.push("C://".to_string());
        files.write().reload_path_list();
    } else {
        let search_results_dom: VirtualDom = VirtualDom::new_with_props(search_results_popup,
        search_results_popupProps { files_props: files.clone(), search_value_props: search_value.clone() });
        window_helper::create_new_dom_generic_window_state(cx, search_results_dom, "Search");
    }
}

#[inline_props]
pub(crate) fn search_results_popup(cx: Scope, files_props: UseRef<Files>, search_value_props: UseState<String>) -> Element {
    let search_results = use_ref(cx,  || Vec::new());

    search_results.write().push(search_value_props.get().to_string());

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
                            // TODO -> This loop is not ready-to-use
                            /*search_results.read().iter().take(search_results.read().len()).map(|path| {
                                let path_borrowed = path.to_string();
                                println!("entrou");

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
                            })*/
                        }
                    }
                }
            }
        }
    ))
}
