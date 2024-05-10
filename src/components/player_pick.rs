use dioxus::prelude::*;
use peppi::game::Player;

use crate::components::character::{CharacterImage, ImageType};

#[component]
pub fn PlayerPick(players: Vec<Player>, on_pick: EventHandler<usize>) -> Element {
    assert!(players.len() > 0);

    // TODO get hint based on previous selected
    let mut selected = use_signal(|| 0);

    let pick = move |evt: FormEvent| async move {
        let id = evt.data.value().parse().expect("value should be index");
        selected.set(id);
        on_pick.call(id);
    };

    rsx! {
        fieldset {
            onchange: pick,
            legend { class: "text-base font-semibold text-gray-900", "Select your player" }
            div { class: "grid md:grid-cols-2 gap-2 mt-2",
                {players.iter().enumerate().map(|(idx, player)| rsx! {
                    label { class: "cursor-pointer rounded-lg border-2 bg-white p-4 hover:border-indigo-200",
                        class: if idx == selected() { "border-indigo-400 hover:border-indigo-400" },
                        input {
                            r#type: "radio",
                            value: "{idx}",
                            name: "player",
                            class: "sr-only"
                        }
                        // TODO better CSS
                        div { class: "grid grid-cols-2 gap-2",
                            span { class: "flex flex-col",
                                svg {
                                    "fill": "currentColor",
                                    "viewBox": "0 0 20 20",
                                    class: "h-5 w-5 text-indigo-600",
                                    class: if idx != selected() { "invisible" },
                                    path {
                                        "d": "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                                        "fill-rule": "evenodd",
                                        "clip-rule": "evenodd"
                                    }
                                }
                                if let Some(netplay) = &player.netplay {
                                    p {
                                        "{netplay.name.to_normalized()}"
                                    }
                                    p { class: "text-sm text-gray-500",
                                        "{netplay.code.to_normalized()}"
                                    }
                                }
                                if let Some(tag) = &player.name_tag {
                                    p {
                                        "{tag.to_normalized()}"
                                    }
                                }
                                p {
                                    class: "justify-end text-xl font-bold",
                                    "{player.port}"
                                }
                            }
                            CharacterImage {
                                character: player.character.try_into().unwrap(),
                                costume: player.costume,
                                image_type: ImageType::SelectMelee,
                            }
                        }
                    }
                })}
            }
        }
    }
}
