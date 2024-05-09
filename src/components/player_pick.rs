use dioxus::prelude::*;
use peppi::game::Player;

#[component]
pub fn PlayerPick(players: Vec<Player>) -> Element {
    assert!(players.len() > 0);
    rsx! {
        fieldset {
            legend { class: "text-base font-semibold text-gray-900", "Select your player" }
            div { class: "mt-4 flex",
                {players.iter().map(|player| rsx! {
                    label { class: "cursor-pointer rounded-lg border bg-white p-4 shadow-sm",
                        input {
                            r#type: "radio",
                            value: "Newsletter",
                            name: "{player.port}",
                            class: "sr-only"
                        }
                        span { class: "flex-col flex-1",
                            span {
                                class: "block text-sm font-medium text-gray-900",
                                "placeholder"
                            }
                            span {
                                class: "mt-1 flex items-center text-sm text-gray-500",
                                "placeholder"
                            }
                            span {
                                class: "mt-6 text-sm font-medium text-gray-900",
                                "{player.port}"
                            }
                        }
                    }
                })}
            }
        }
    }
}

// check svg
// svg {
//     "fill": "currentColor",
//     "aria-hidden": "true",
//     "viewBox": "0 0 20 20",
//     class: "h-5 w-5 text-indigo-600",
//     path {
//         "d": "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
//         "fill-rule": "evenodd",
//         "clip-rule": "evenodd"
//     }
// }
