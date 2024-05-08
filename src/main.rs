#![allow(non_snake_case)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
use tracing::Level;

mod components {
    pub mod replay_drop;
    pub mod sidebar;
}
use components::replay_drop::ReplayDrop;
use components::sidebar::SideBar;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(SideBar)]
        #[route("/")]
        Home {},
        #[route("/technique")]
        Technique {},
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
fn Technique() -> Element {
    rsx! {
        p { "technique info" }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        ReplayDrop {
            on_slp: move |rp| println!("received {}", rp)
        }
    }
}
