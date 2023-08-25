use std::sync::Mutex;
use dioxus::core::{Event};
use dioxus::events::KeyboardData;
use dioxus::hooks::UseRef;
use dioxus::html::input_data::keyboard_types::{Code, Modifiers};
use dioxus::prelude::Scope;

use crate::Files;
use crate::rename_operation::rename_event;

pub fn handle_keydown_event(context: Scope, keydown_event: Event<KeyboardData>, files: &UseRef<Files>, clicked_directory_id: &Mutex<usize>) {
    if keydown_event.modifiers().contains(Modifiers::CONTROL) && keydown_event.inner().code() == Code::KeyR {
        rename_event(context, files, clicked_directory_id);
    }
    // todo
}
