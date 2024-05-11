use dioxus::prelude::*;
use manganis::ImageAsset;
use ssbm_data::character::External;

#[derive(Props, PartialEq, Clone)]
pub struct CharacterImageProps {
    character: External,
    image_type: ImageType,
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[derive(PartialEq, Clone)]
pub enum ImageType {
    Portrait,
    //Stock(u8),
    Css(u8),
    Render(u8),
}

#[component]
pub fn CharacterImage(props: CharacterImageProps) -> Element {
    let asset = match props.character {
        External::CaptainFalcon => FALCON.get(props.image_type),
        External::DonkeyKong => DK.get(props.image_type),
        External::Fox => FOX.get(props.image_type),
        External::GameAndWatch => GNW.get(props.image_type),
        External::Kirby => KIRBY.get(props.image_type),
        External::Bowser => BOWSER.get(props.image_type),
        External::Link => LINK.get(props.image_type),
        External::Luigi => LUIGI.get(props.image_type),
        External::Mario => MARIO.get(props.image_type),
        External::Marth => MARTH.get(props.image_type),
        External::Mewtwo => MEWTWO.get(props.image_type),
        External::Ness => NESS.get(props.image_type),
        External::Peach => PEACH.get(props.image_type),
        External::Pikachu => PIKACHU.get(props.image_type),
        External::IceClimbers => ICS.get(props.image_type),
        External::Jigglypuff => JIGGLYPUFF.get(props.image_type),
        External::Samus => SAMUS.get(props.image_type),
        External::Yoshi => YOSHI.get(props.image_type),
        External::Zelda => ZELDA.get(props.image_type),
        External::Sheik => SHEIK.get(props.image_type),
        External::Falco => FALCO.get(props.image_type),
        External::YoungLink => YLINK.get(props.image_type),
        External::DrMario => DOC.get(props.image_type),
        External::Roy => ROY.get(props.image_type),
        External::Pichu => PICHU.get(props.image_type),
        External::Ganondorf => GANONDORF.get(props.image_type),
        _ => FALCON.get(props.image_type),
    };

    rsx! {
        img {
            src: "{asset}",
            ..props.attributes
        }
    }
}

struct CharacterImages<const N: usize> {
    portrait: ImageAsset,
    //stock: [ImageAsset; N],
    css: [ImageAsset; N],
    render: [ImageAsset; N],
}

impl<const N: usize> CharacterImages<N> {
    fn get(&self, r#type: ImageType) -> ImageAsset {
        match r#type {
            ImageType::Portrait => self.portrait.clone(),
            ImageType::Css(n) => self.css.get(n as usize).unwrap_or(&self.css[0]).clone(),
            ImageType::Render(n) => self
                .render
                .get(n as usize)
                .unwrap_or(&self.render[0])
                .clone(),
        }
    }
}

const FALCON: CharacterImages<6> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/falcon/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/falcon/css/1.png")),
        manganis::mg!(image("assets/melee/character/falcon/css/2.png")),
        manganis::mg!(image("assets/melee/character/falcon/css/3.png")),
        manganis::mg!(image("assets/melee/character/falcon/css/4.png")),
        manganis::mg!(image("assets/melee/character/falcon/css/5.png")),
        manganis::mg!(image("assets/melee/character/falcon/css/6.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/falcon/renders/1.png")),
        manganis::mg!(image("assets/melee/character/falcon/renders/2.png")),
        manganis::mg!(image("assets/melee/character/falcon/renders/3.png")),
        manganis::mg!(image("assets/melee/character/falcon/renders/4.png")),
        manganis::mg!(image("assets/melee/character/falcon/renders/5.png")),
        manganis::mg!(image("assets/melee/character/falcon/renders/6.png")),
    ],
};

const DK: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/dk/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/dk/css/1.png")),
        manganis::mg!(image("assets/melee/character/dk/css/2.png")),
        manganis::mg!(image("assets/melee/character/dk/css/3.png")),
        manganis::mg!(image("assets/melee/character/dk/css/4.png")),
        manganis::mg!(image("assets/melee/character/dk/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/dk/renders/1.png")),
        manganis::mg!(image("assets/melee/character/dk/renders/2.png")),
        manganis::mg!(image("assets/melee/character/dk/renders/3.png")),
        manganis::mg!(image("assets/melee/character/dk/renders/4.png")),
        manganis::mg!(image("assets/melee/character/dk/renders/5.png")),
    ],
};

const FOX: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/fox/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/fox/css/1.png")),
        manganis::mg!(image("assets/melee/character/fox/css/2.png")),
        manganis::mg!(image("assets/melee/character/fox/css/3.png")),
        manganis::mg!(image("assets/melee/character/fox/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/fox/renders/1.png")),
        manganis::mg!(image("assets/melee/character/fox/renders/2.png")),
        manganis::mg!(image("assets/melee/character/fox/renders/3.png")),
        manganis::mg!(image("assets/melee/character/fox/renders/4.png")),
    ],
};

const GNW: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/gnw/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/gnw/css/1.png")),
        manganis::mg!(image("assets/melee/character/gnw/css/2.png")),
        manganis::mg!(image("assets/melee/character/gnw/css/3.png")),
        manganis::mg!(image("assets/melee/character/gnw/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/gnw/renders/1.png")),
        manganis::mg!(image("assets/melee/character/gnw/renders/2.png")),
        manganis::mg!(image("assets/melee/character/gnw/renders/3.png")),
        manganis::mg!(image("assets/melee/character/gnw/renders/4.png")),
    ],
};

const KIRBY: CharacterImages<6> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/kirby/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/kirby/css/1.png")),
        manganis::mg!(image("assets/melee/character/kirby/css/2.png")),
        manganis::mg!(image("assets/melee/character/kirby/css/3.png")),
        manganis::mg!(image("assets/melee/character/kirby/css/4.png")),
        manganis::mg!(image("assets/melee/character/kirby/css/5.png")),
        manganis::mg!(image("assets/melee/character/kirby/css/6.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/kirby/renders/1.png")),
        manganis::mg!(image("assets/melee/character/kirby/renders/2.png")),
        manganis::mg!(image("assets/melee/character/kirby/renders/3.png")),
        manganis::mg!(image("assets/melee/character/kirby/renders/4.png")),
        manganis::mg!(image("assets/melee/character/kirby/renders/5.png")),
        manganis::mg!(image("assets/melee/character/kirby/renders/6.png")),
    ],
};

const BOWSER: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/bowser/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/bowser/css/1.png")),
        manganis::mg!(image("assets/melee/character/bowser/css/2.png")),
        manganis::mg!(image("assets/melee/character/bowser/css/3.png")),
        manganis::mg!(image("assets/melee/character/bowser/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/bowser/renders/1.png")),
        manganis::mg!(image("assets/melee/character/bowser/renders/2.png")),
        manganis::mg!(image("assets/melee/character/bowser/renders/3.png")),
        manganis::mg!(image("assets/melee/character/bowser/renders/4.png")),
    ],
};

const LINK: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/link/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/link/css/1.png")),
        manganis::mg!(image("assets/melee/character/link/css/2.png")),
        manganis::mg!(image("assets/melee/character/link/css/3.png")),
        manganis::mg!(image("assets/melee/character/link/css/4.png")),
        manganis::mg!(image("assets/melee/character/link/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/link/renders/1.png")),
        manganis::mg!(image("assets/melee/character/link/renders/2.png")),
        manganis::mg!(image("assets/melee/character/link/renders/3.png")),
        manganis::mg!(image("assets/melee/character/link/renders/4.png")),
        manganis::mg!(image("assets/melee/character/link/renders/5.png")),
    ],
};

const LUIGI: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/luigi/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/luigi/css/1.png")),
        manganis::mg!(image("assets/melee/character/luigi/css/2.png")),
        manganis::mg!(image("assets/melee/character/luigi/css/3.png")),
        manganis::mg!(image("assets/melee/character/luigi/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/luigi/renders/1.png")),
        manganis::mg!(image("assets/melee/character/luigi/renders/2.png")),
        manganis::mg!(image("assets/melee/character/luigi/renders/3.png")),
        manganis::mg!(image("assets/melee/character/luigi/renders/4.png")),
    ],
};

const MARIO: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/mario/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/mario/css/1.png")),
        manganis::mg!(image("assets/melee/character/mario/css/2.png")),
        manganis::mg!(image("assets/melee/character/mario/css/3.png")),
        manganis::mg!(image("assets/melee/character/mario/css/4.png")),
        manganis::mg!(image("assets/melee/character/mario/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/mario/renders/1.png")),
        manganis::mg!(image("assets/melee/character/mario/renders/2.png")),
        manganis::mg!(image("assets/melee/character/mario/renders/3.png")),
        manganis::mg!(image("assets/melee/character/mario/renders/4.png")),
        manganis::mg!(image("assets/melee/character/mario/renders/5.png")),
    ],
};

const MARTH: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/marth/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/marth/css/1.png")),
        manganis::mg!(image("assets/melee/character/marth/css/2.png")),
        manganis::mg!(image("assets/melee/character/marth/css/3.png")),
        manganis::mg!(image("assets/melee/character/marth/css/4.png")),
        manganis::mg!(image("assets/melee/character/marth/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/marth/renders/1.png")),
        manganis::mg!(image("assets/melee/character/marth/renders/2.png")),
        manganis::mg!(image("assets/melee/character/marth/renders/3.png")),
        manganis::mg!(image("assets/melee/character/marth/renders/4.png")),
        manganis::mg!(image("assets/melee/character/marth/renders/5.png")),
    ],
};

const MEWTWO: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/mewtwo/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/mewtwo/css/1.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/css/2.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/css/3.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/mewtwo/renders/1.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/renders/2.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/renders/3.png")),
        manganis::mg!(image("assets/melee/character/mewtwo/renders/4.png")),
    ],
};

const NESS: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/ness/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/ness/css/1.png")),
        manganis::mg!(image("assets/melee/character/ness/css/2.png")),
        manganis::mg!(image("assets/melee/character/ness/css/3.png")),
        manganis::mg!(image("assets/melee/character/ness/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/ness/renders/1.png")),
        manganis::mg!(image("assets/melee/character/ness/renders/2.png")),
        manganis::mg!(image("assets/melee/character/ness/renders/3.png")),
        manganis::mg!(image("assets/melee/character/ness/renders/4.png")),
    ],
};

const PEACH: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/peach/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/peach/css/1.png")),
        manganis::mg!(image("assets/melee/character/peach/css/2.png")),
        manganis::mg!(image("assets/melee/character/peach/css/3.png")),
        manganis::mg!(image("assets/melee/character/peach/css/4.png")),
        manganis::mg!(image("assets/melee/character/peach/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/peach/renders/1.png")),
        manganis::mg!(image("assets/melee/character/peach/renders/2.png")),
        manganis::mg!(image("assets/melee/character/peach/renders/3.png")),
        manganis::mg!(image("assets/melee/character/peach/renders/4.png")),
        manganis::mg!(image("assets/melee/character/peach/renders/5.png")),
    ],
};

const PIKACHU: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/pikachu/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/pikachu/css/1.png")),
        manganis::mg!(image("assets/melee/character/pikachu/css/2.png")),
        manganis::mg!(image("assets/melee/character/pikachu/css/3.png")),
        manganis::mg!(image("assets/melee/character/pikachu/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/pikachu/renders/1.png")),
        manganis::mg!(image("assets/melee/character/pikachu/renders/2.png")),
        manganis::mg!(image("assets/melee/character/pikachu/renders/3.png")),
        manganis::mg!(image("assets/melee/character/pikachu/renders/4.png")),
    ],
};

const ICS: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/ics/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/ics/css/1.png")),
        manganis::mg!(image("assets/melee/character/ics/css/2.png")),
        manganis::mg!(image("assets/melee/character/ics/css/3.png")),
        manganis::mg!(image("assets/melee/character/ics/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/ics/renders/1.png")),
        manganis::mg!(image("assets/melee/character/ics/renders/2.png")),
        manganis::mg!(image("assets/melee/character/ics/renders/3.png")),
        manganis::mg!(image("assets/melee/character/ics/renders/4.png")),
    ],
};

const JIGGLYPUFF: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/jigglypuff/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/jigglypuff/css/1.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/css/2.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/css/3.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/css/4.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/jigglypuff/renders/1.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/renders/2.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/renders/3.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/renders/4.png")),
        manganis::mg!(image("assets/melee/character/jigglypuff/renders/5.png")),
    ],
};

const SAMUS: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/samus/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/samus/css/1.png")),
        manganis::mg!(image("assets/melee/character/samus/css/2.png")),
        manganis::mg!(image("assets/melee/character/samus/css/3.png")),
        manganis::mg!(image("assets/melee/character/samus/css/4.png")),
        manganis::mg!(image("assets/melee/character/samus/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/samus/renders/1.png")),
        manganis::mg!(image("assets/melee/character/samus/renders/2.png")),
        manganis::mg!(image("assets/melee/character/samus/renders/3.png")),
        manganis::mg!(image("assets/melee/character/samus/renders/4.png")),
        manganis::mg!(image("assets/melee/character/samus/renders/5.png")),
    ],
};

const YOSHI: CharacterImages<6> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/yoshi/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/yoshi/css/1.png")),
        manganis::mg!(image("assets/melee/character/yoshi/css/2.png")),
        manganis::mg!(image("assets/melee/character/yoshi/css/3.png")),
        manganis::mg!(image("assets/melee/character/yoshi/css/4.png")),
        manganis::mg!(image("assets/melee/character/yoshi/css/5.png")),
        manganis::mg!(image("assets/melee/character/yoshi/css/6.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/yoshi/renders/1.png")),
        manganis::mg!(image("assets/melee/character/yoshi/renders/2.png")),
        manganis::mg!(image("assets/melee/character/yoshi/renders/3.png")),
        manganis::mg!(image("assets/melee/character/yoshi/renders/4.png")),
        manganis::mg!(image("assets/melee/character/yoshi/renders/5.png")),
        manganis::mg!(image("assets/melee/character/yoshi/renders/6.png")),
    ],
};

const ZELDA: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/zelda/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/zelda/css/1.png")),
        manganis::mg!(image("assets/melee/character/zelda/css/2.png")),
        manganis::mg!(image("assets/melee/character/zelda/css/3.png")),
        manganis::mg!(image("assets/melee/character/zelda/css/4.png")),
        manganis::mg!(image("assets/melee/character/zelda/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/zelda/renders/1.png")),
        manganis::mg!(image("assets/melee/character/zelda/renders/2.png")),
        manganis::mg!(image("assets/melee/character/zelda/renders/3.png")),
        manganis::mg!(image("assets/melee/character/zelda/renders/4.png")),
        manganis::mg!(image("assets/melee/character/zelda/renders/5.png")),
    ],
};

const SHEIK: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/sheik/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/sheik/css/1.png")),
        manganis::mg!(image("assets/melee/character/sheik/css/2.png")),
        manganis::mg!(image("assets/melee/character/sheik/css/3.png")),
        manganis::mg!(image("assets/melee/character/sheik/css/4.png")),
        manganis::mg!(image("assets/melee/character/sheik/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/sheik/renders/1.png")),
        manganis::mg!(image("assets/melee/character/sheik/renders/2.png")),
        manganis::mg!(image("assets/melee/character/sheik/renders/3.png")),
        manganis::mg!(image("assets/melee/character/sheik/renders/4.png")),
        manganis::mg!(image("assets/melee/character/sheik/renders/5.png")),
    ],
};

const FALCO: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/falco/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/falco/css/1.png")),
        manganis::mg!(image("assets/melee/character/falco/css/2.png")),
        manganis::mg!(image("assets/melee/character/falco/css/3.png")),
        manganis::mg!(image("assets/melee/character/falco/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/falco/renders/1.png")),
        manganis::mg!(image("assets/melee/character/falco/renders/2.png")),
        manganis::mg!(image("assets/melee/character/falco/renders/3.png")),
        manganis::mg!(image("assets/melee/character/falco/renders/4.png")),
    ],
};

const YLINK: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/ylink/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/ylink/css/1.png")),
        manganis::mg!(image("assets/melee/character/ylink/css/2.png")),
        manganis::mg!(image("assets/melee/character/ylink/css/3.png")),
        manganis::mg!(image("assets/melee/character/ylink/css/4.png")),
        manganis::mg!(image("assets/melee/character/ylink/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/ylink/renders/1.png")),
        manganis::mg!(image("assets/melee/character/ylink/renders/2.png")),
        manganis::mg!(image("assets/melee/character/ylink/renders/3.png")),
        manganis::mg!(image("assets/melee/character/ylink/renders/4.png")),
        manganis::mg!(image("assets/melee/character/ylink/renders/5.png")),
    ],
};

const DOC: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/doc/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/doc/css/1.png")),
        manganis::mg!(image("assets/melee/character/doc/css/2.png")),
        manganis::mg!(image("assets/melee/character/doc/css/3.png")),
        manganis::mg!(image("assets/melee/character/doc/css/4.png")),
        manganis::mg!(image("assets/melee/character/doc/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/doc/renders/1.png")),
        manganis::mg!(image("assets/melee/character/doc/renders/2.png")),
        manganis::mg!(image("assets/melee/character/doc/renders/3.png")),
        manganis::mg!(image("assets/melee/character/doc/renders/4.png")),
        manganis::mg!(image("assets/melee/character/doc/renders/5.png")),
    ],
};

const ROY: CharacterImages<5> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/roy/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/roy/css/1.png")),
        manganis::mg!(image("assets/melee/character/roy/css/2.png")),
        manganis::mg!(image("assets/melee/character/roy/css/3.png")),
        manganis::mg!(image("assets/melee/character/roy/css/4.png")),
        manganis::mg!(image("assets/melee/character/roy/css/5.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/roy/renders/1.png")),
        manganis::mg!(image("assets/melee/character/roy/renders/2.png")),
        manganis::mg!(image("assets/melee/character/roy/renders/3.png")),
        manganis::mg!(image("assets/melee/character/roy/renders/4.png")),
        manganis::mg!(image("assets/melee/character/roy/renders/5.png")),
    ],
};

const PICHU: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/pichu/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/pichu/css/1.png")),
        manganis::mg!(image("assets/melee/character/pichu/css/2.png")),
        manganis::mg!(image("assets/melee/character/pichu/css/3.png")),
        manganis::mg!(image("assets/melee/character/pichu/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/pichu/renders/1.png")),
        manganis::mg!(image("assets/melee/character/pichu/renders/2.png")),
        manganis::mg!(image("assets/melee/character/pichu/renders/3.png")),
        manganis::mg!(image("assets/melee/character/pichu/renders/4.png")),
    ],
};

const GANONDORF: CharacterImages<4> = CharacterImages {
    portrait: manganis::mg!(image("assets/melee/character/ganondorf/portrait.png")),
    css: [
        manganis::mg!(image("assets/melee/character/ganondorf/css/1.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/css/2.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/css/3.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/css/4.png")),
    ],
    render: [
        manganis::mg!(image("assets/melee/character/ganondorf/renders/1.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/renders/2.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/renders/3.png")),
        manganis::mg!(image("assets/melee/character/ganondorf/renders/4.png")),
    ],
};
