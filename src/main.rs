#![allow(non_snake_case)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
use tracing::Level;

mod components {
    pub mod button;
    pub mod sidebar;
}
use components::sidebar::SideBar;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
// const _STYLE: &str = {
//     const _: &dyn manganis::ForMgMacro = {
//         use manganis::*;
//         &file("public/tailwind.css")
//     };
//     "/tailwindcssfe277c00ea35019e.css"
// };

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(SideBar)]
        #[route("/")]
        Home {},
        #[route("/wavedash")]
        Wavedash {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new().with_window(
        WindowBuilder::new()
            .with_title("Ledgedash")
            .with_inner_size(LogicalSize::new(800, 700)),
    );
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Wavedash() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full",
            "wavedash info"
        }
    }
}

#[component]
fn Home() -> Element {
    let mut search_subdirs = use_signal(|| true);
    let mut num = use_signal(|| 0usize);
    let mut name = use_signal(|| None as Option<String>);

    let get_replays = move |evt: FormEvent| async move {
        let Some(file_engine) = evt.files() else {
            panic!()
        };
        let files = file_engine.files();
        name.set(files.get(0).cloned());
        num.set(files.len());
    };

    rsx! {
        h2 { "{num}" }
        if let Some(name) = name() {
            h2 { {name} }
        }
        div {
            class: "grid grid-cols-2",
            div {
                label { r#for: "upload_replays", "Upload .slp replay files" }
                input {
                    class: "block",
                    r#type: "file",
                    accept: ".slp",
                    multiple: true,
                    name: "upload_replays",
                    onchange: get_replays,
                }
            }
            div {
                label { r#for: "upload_replay_folder", "Upload folder containing replays" }
                input {
                    class: "block",
                    r#type: "file",
                    directory: true,
                    name: "upload_replay_folder",
                    onchange: get_replays,
                }
                label { r#for: "search_subdirs", "Search Subdirectories" }
                input {
                    class: "block",
                    r#type: "checkbox",
                    id: "search_subdirs",
                    checked: search_subdirs,
                    oninput: move |evt| search_subdirs.set(evt.checked()),
                }
            }
        }
    }
}
