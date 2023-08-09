use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_desktop::tao::platform::windows::IconExtWindows;
use dioxus_desktop::tao::window::Icon;

fn main() {
    let icon: Option<Icon>;

    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(800.0, 400.0))
            //.with_window_icon(Icon::from_path(dioxus_free_icons::IconShape))
            .with_theme(Option::from(dioxus_desktop::tao::window::Theme::Dark))
        )
    );
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
       div { "Hello, World!" }
    ))
}
