use dioxus::html::input_data::keyboard_types::Code;
use dioxus::prelude::*;
use crate::Files;

pub(crate) fn create_search_input_field(cx: Scope, files: &UseRef<Files>, is_search_field_enabled: &UseState<bool>) -> LazyNodes {
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
                placeholder: "Type the path/or archive name...",
                oninput: |type_event| {
                    search_value.set(type_event.value.to_string());
                },
                onkeydown: |keydown_event| {
                    if keydown_event.inner().code() == Code::Enter {
                        execute_search_operation(files, search_value);
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

fn execute_search_operation(files: &UseRef<Files>, is_search_field_enabled: &UseState<String>) {

}
