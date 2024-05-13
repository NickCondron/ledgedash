#![allow(non_snake_case)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
use peppi::{game::immutable::Game, io::slippi};
use std::io::Cursor;
use tracing::Level;

pub mod components {
    pub mod character;
    pub mod player_pick;
    pub mod replay_drop;
    pub mod sidebar;
}
use components::player_pick::PlayerPick;
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
    use_context_provider(|| Signal::new(None as Option<Game>));
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
    let mut slp = use_signal(|| None as Option<String>);
    let parse_game_future: Resource<Result<Option<Game>, peppi::io::Error>> =
        use_resource(move || async move {
            let Some(slp) = slp() else {
                return Ok(None);
            };
            // let slp_bytes = tokio::fs::read(slp).await.map_err(peppi::io::Error::Io)?;
            let slp_bytes = std::fs::read(slp).map_err(peppi::io::Error::Io)?;
            let game = slippi::de::read(Cursor::new(slp_bytes), None)?;
            Ok(Some(game))
        });

    rsx! {
        ReplayDrop {
            on_slp: move |rp| {
                println!("received {}", rp);
                slp.set(Some(rp));
            }
        }
        match &*parse_game_future.read_unchecked() {
            Some(Ok(None)) => None, // no replay provided by user
            Some(Ok(Some(game))) => rsx! {
                PlayerPick {
                    players: game.start.players.clone(),
                    on_pick: move |p| println!("player {} selected", p)
                }
            },
            Some(Err(_err)) => rsx! {
                "parse error"
            },
            None => rsx! {
                "app is processing replay"
            },
        }
    }
}
