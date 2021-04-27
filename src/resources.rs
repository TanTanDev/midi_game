use crate::input::*;
use macroquad::audio::*;
use macroquad::prelude::*;
use macroquad_tantan_toolbox::resources::*;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum TransitionData {
    Slide,
}

impl Default for TransitionData {
    fn default() -> Self {
        TransitionData::Slide
    }
}

pub struct SharedData {
    pub texture_resources_optional: Option<TextureResources>,
    pub raw_image_resources_optional: Option<RawImageResources>,
    pub sound_resources_optional: Option<SoundResources>,
    pub input: Input,
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum TextureIdentifier {
    Player,
    Scenery,
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum RawImageResourceIdentifier {
    WaterNormal,
}

pub struct TextureResources {
    pub player: Texture2D,
    pub scenery: Texture2D,
}

pub struct RawImageResources {
    // optional so we can consume it
    pub water_normal: Option<Image>,
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum SoundIdentifier {
    Jump,
    Jammed,
    LatchOpen,
    LatchClose,
    Shoot,
    Crouch,
    Uncrouch,
}

pub struct SoundResources {
    // optional so we can consume it
    pub jump: Sound,
    pub latch_open: Sound,
    pub latch_close: Sound,
    pub jammed: Sound,
    pub shoot: Sound,
    pub crouch: Sound,
    pub uncrouch: Sound,
}

impl Resources<SoundIdentifier, Sound, DefaultFactory> for SoundResources {
    fn build(builder: &mut ResourceBuilder<SoundIdentifier, Self, Sound, DefaultFactory>) -> Self {
        Self {
            jump: builder.get_or_panic(SoundIdentifier::Jump),
            latch_close: builder.get_or_panic(SoundIdentifier::LatchClose),
            latch_open: builder.get_or_panic(SoundIdentifier::LatchOpen),
            jammed: builder.get_or_panic(SoundIdentifier::Jammed),
            shoot: builder.get_or_panic(SoundIdentifier::Shoot),
            crouch: builder.get_or_panic(SoundIdentifier::Crouch),
            uncrouch: builder.get_or_panic(SoundIdentifier::Uncrouch),
        }
    }
}

impl Resources<TextureIdentifier, Texture2D, DefaultFactory> for TextureResources {
    fn build(
        builder: &mut ResourceBuilder<TextureIdentifier, Self, Texture2D, DefaultFactory>,
    ) -> Self {
        Self {
            player: builder.get_or_panic(TextureIdentifier::Player),
            scenery: builder.get_or_panic(TextureIdentifier::Scenery),
        }
    }
}

impl Resources<RawImageResourceIdentifier, Image, DefaultFactory> for RawImageResources {
    fn build(
        builder: &mut ResourceBuilder<RawImageResourceIdentifier, Self, Image, DefaultFactory>,
    ) -> Self {
        Self {
            water_normal: Some(builder.get_or_panic(RawImageResourceIdentifier::WaterNormal)),
        }
    }
}
