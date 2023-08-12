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
        style { include_str!("./assets/styles.css")  }
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
        header {
            i { class: "material-icons icon-menu", "menu" }
            h1 { "Files: ", /*files.read().current()*/ }
            span { }
            i { class: "material-icons", onclick: move |_| close_application(&cx), "logout" }
        }
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

fn close_application(cx: Scope) {
    let window = dioxus_desktop::use_window(&cx);
    window.close_window(window.id());
}
