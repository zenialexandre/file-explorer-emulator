use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use image::GenericImageView;
use dioxus_desktop::tao::window::Icon as TaoIcon;

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(800.0, 400.0))
            .with_window_icon(load_icon_by_path("src/images/icon/simple_face_icon.png"))
            .with_theme(Option::from(dioxus_desktop::tao::window::Theme::Dark))
        )
    );
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
       div { "Hello, World!" }
    ))
}

fn load_icon_by_path(file_path: &str) -> Option<TaoIcon> {
     return if let Ok(image) = image::open(file_path) {
        let (width, height) = image.dimensions();
        let rgba_data = image.to_rgba8().into_raw();
        Some(TaoIcon::from_rgba(rgba_data, width, height).expect("Failed to load icon."))
    } else {
        None
    }
}
