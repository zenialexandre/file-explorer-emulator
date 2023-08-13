use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use image::GenericImageView;
use dioxus_desktop::tao::window::Icon as TaoIcon;

struct Files {
    path_stack: Vec<String>,
    path_names: Vec<String>,
    error: Option<String>,
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new()
            .with_resizable(true).with_title("File Explorer Emulator")
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(1000.0, 800.0))
            .with_window_icon(load_icon_by_path("src/images/icon/simple_face_icon.png"))
            .with_theme(Option::from(dioxus_desktop::tao::window::Theme::Dark))
        )
    );
}

fn app(cx: Scope) -> Element {
    let files = use_ref(cx, Files::new);

    cx.render(rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet", }
            style { include_str!("./assets/styles.css") }
            header {
                i { class: "material-icons", onclick: move |_| files.write().go_up(), "arrow_back" }
                i { class: "material-icons", "arrow_forward" }
                h1 { "Files: ", files.read().current() }
                span { }
                i { class: "material-icons", onclick: move |_| close_application(cx), "cancel" }
            }
            main {
                files.read().path_names.iter().enumerate().map(|(directory_id, path)| {
                    let path_end = path.split('/').last().unwrap_or(path.as_str());
                    let icon_type = if path_end.contains('.') {
                        "description"
                    } else {
                        "folder"
                    };
                    rsx! (
                        div {
                            class: "folder",
                            key: "{path}",
                            i { class: "material-icons", ondblclick: move |_| files.write().enter_directory(directory_id), "{icon_type}"
                                //p { class: "cooltip", "0 folders / 0 files" }
                            }
                            h1 { "{path_end}" }
                        }
                    )
                }),
                files.read().error.as_ref().map(|err| {
                    rsx! (
                        div {
                            code { "{err}" }
                            button { onclick: move |_| files.write().clear_error(), "x" }
                        }
                    )
                })
            }
        }
    })
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

impl Files {
    fn new() -> Self {
      let mut files = Self {
          path_stack: vec!["./".to_string()],
          path_names: vec![],
          error: None,
      };
      files.reload_path_list();
      files
    }

    fn reload_path_list(&mut self) {
        let current_path = self.path_stack.last().unwrap();
        let paths = match std::fs::read_dir(current_path) {
            Ok(e) => e,
            Err(error) => {
                let error = format!("An error occurred: {error:?}");
                self.error = Some(error);
                self.path_stack.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();

        // clear current state
        self.clear_error();
        self.path_names.clear();

        for path in collected {
            self.path_names.push(path.unwrap().path().display().to_string());
        }
    }

    fn go_up(&mut self) {
        if self.path_stack.len() > 1 {
            self.path_stack.pop();
        }
        self.reload_path_list();
    }

    fn enter_directory(&mut self, directory_id: usize) {
        let path = &self.path_names[directory_id];
        self.path_stack.push(path.clone());
        self.reload_path_list();
    }

    fn current(&self) -> &str {
        return self.path_stack.last().unwrap();
    }

    fn clear_error(&mut self) {
        self.error = None;
    }
}
