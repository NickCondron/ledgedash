#![allow(non_snake_case)]

use dioxus::{
    desktop::{LogicalSize, WindowBuilder},
    prelude::*,
};
use tracing::Level;

mod components {
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
    let mock_player1 = peppi::game::Player {
        port: peppi::game::Port::P1,
        character: 1,
        r#type: peppi::game::PlayerType::Human,
        stocks: 4,
        costume: 1,
        team: None,
        handicap: 1,
        bitfield: 0,
        cpu_level: None,
        offense_ratio: 1.0f32,
        defense_ratio: 1.0f32,
        model_scale: 1.0f32,
        ucf: None,
        name_tag: None,
        netplay: Some(peppi::game::Netplay {
            name: peppi::game::shift_jis::MeleeString("Clown".to_string()),
            code: peppi::game::shift_jis::MeleeString("CLWN#889".to_string()),
            suid: None,
        }),
    };

    let mock_player2 = peppi::game::Player {
        port: peppi::game::Port::P2,
        character: 5,
        r#type: peppi::game::PlayerType::Human,
        stocks: 4,
        costume: 2,
        team: None,
        handicap: 1,
        bitfield: 0,
        cpu_level: None,
        offense_ratio: 1.0f32,
        defense_ratio: 1.0f32,
        model_scale: 1.0f32,
        ucf: None,
        name_tag: Some(peppi::game::shift_jis::MeleeString("KUSH".to_string())),
        netplay: None,
    };

    let mut players = Vec::new();
    players.push(mock_player1);
    players.push(mock_player2);
    rsx! {
        ReplayDrop {
            on_slp: move |rp| println!("received {}", rp)
        }
        PlayerPick {
            players: players
        }
    }
}
