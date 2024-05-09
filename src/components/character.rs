use dioxus::prelude::*;
use manganis::ImageAsset;
use ssbm_data::character::External;

#[derive(Props, PartialEq, Clone)]
pub struct CharacterImageProps {
    character: External,
    costume: u8,
    image_type: ImageType,
    #[props(extends = img)]
    attributes: Vec<Attribute>,
}

#[derive(PartialEq, Clone)]
pub enum ImageType {
    SelectMelee,
}

#[component]
pub fn CharacterImage(props: CharacterImageProps) -> Element {
    let costume_list = match props.character {
        External::CaptainFalcon => &FALCON,
        External::DonkeyKong => &FALCON,
        External::Fox => &FALCON,
        External::GameAndWatch => &FALCON,
        External::Kirby => &FALCON,
        External::Bowser => &FALCON,
        External::Link => &FALCON,
        External::Luigi => &FALCON,
        External::Mario => &FALCON,
        External::Marth => &FALCON,
        External::Mewtwo => &FALCON,
        External::Ness => &FALCON,
        External::Peach => &FALCON,
        External::Pikachu => &FALCON,
        External::IceClimbers => &FALCON,
        External::Jigglypuff => &FALCON,
        External::Samus => &FALCON,
        External::Yoshi => &FALCON,
        External::Zelda => &FALCON,
        External::Sheik => &FALCON,
        External::Falco => &FALCON,
        External::YoungLink => &FALCON,
        External::DrMario => &FALCON,
        External::Roy => &FALCON,
        External::Pichu => &FALCON,
        External::Ganondorf => &FALCON,
        _ => &FALCON,
    };

    let costume = costume_list
        .get(props.costume as usize)
        .unwrap_or(&costume_list[0]);
    let image = match props.image_type {
        ImageType::SelectMelee => costume.css_melee.clone(),
    };

    rsx! {
        img {
            src: image,
            ..props.attributes
        }
    }
}

struct CostumeImages {
    css_melee: ImageAsset,
}

const FALCON_INDIGO: CostumeImages = CostumeImages {
    css_melee: manganis::mg!(image("assets/melee/renders/falcon/melee/css/1.png")),
};
const FALCON_BLACK: CostumeImages = CostumeImages {
    css_melee: manganis::mg!(image("assets/melee/renders/falcon/melee/css/2.png")),
};

const FALCON: [CostumeImages; 2] = [FALCON_INDIGO, FALCON_BLACK];
