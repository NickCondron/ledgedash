use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons;
use dioxus_free_icons::Icon;

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div { class: "fixed inset-y-0 left-0 z-50 flex w-72 flex-col",
            div { class: "flex grow flex-col overflow-y-auto bg-gray-900 px-6",
                div { class: "flex h-16 items-center",
                    img { class: "h-12 w-auto",
                        src: manganis::mg!(image("assets/ledgedash.png"))
                    }
                }
                nav { class: "flex flex-1 flex-col",
                    ul { class: "-mx-2 space-y-1",
                        li {
                            Link {
                                to: Route::Home {},
                                //class: "bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                class: "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                Icon {
                                    icon: bs_icons::BsHouse,
                                }
                                "Dashboard"
                            }
                        }
                        li {
                            Link {
                                to: Route::Technique {},
                                class: "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold",
                                Icon {
                                    icon: bs_icons::BsBrush,
                                }
                                "Technique"
                            }
                        }
                    }
                }
            }
        }
        main {
            class: "pl-72",
            div {
                class: "p-4",
                Outlet::<Route> {}
            }
        }
    }
}
