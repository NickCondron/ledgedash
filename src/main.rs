#![allow(non_snake_case)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};

use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/wavedash")]
        Wavedash {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
        .with_window(
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
        input {
            r#type: "file",
            accept: ".slp",
            multiple: true,
            onchange: get_replays,
        }
        input {
            r#type: "file",
            directory: true,
            onchange: get_replays,
        }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            class: "bg-gray-800",
            ul {
                class: "flex",
                li {
                    class: "mr-6",
                    Link {
                        class: "text-blue-500 hover:text-blue-800",
                        to: Route::Home {},
                        "Home"
                    }
                }
                li {
                    class: "mr-6",
                    Link {
                        class: "text-blue-500 hover:text-blue-800",
                        to: Route::Wavedash {},
                        "Wavedash"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
