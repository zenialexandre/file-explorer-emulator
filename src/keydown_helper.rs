use std::sync::Mutex;
use dioxus::core::Event;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::html::KeyboardData;

pub fn handle_keydown_event(keydown_event: Event<KeyboardData>, current_path: String, clicked_directory_id: &Mutex<usize>) {
    if keydown_event.inner().code() == Code::F9 {
        println!("aaa");
    }
}
