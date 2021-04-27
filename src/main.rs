use crate::constants::*;
use crate::gun::*;
use crate::input::*;
use crate::player::*;
use crate::resources::*;
use crate::water::*;
use async_trait::async_trait;
use macroquad::audio::*;
use macroquad::prelude::*;
use macroquad_tantan_toolbox::animation::*;
use macroquad_tantan_toolbox::resources::*;
use macroquad_tantan_toolbox::states::*;
use std::collections::HashMap;

mod constants;
mod gun;
mod input;
mod player;
mod resources;
mod water;

pub struct GameStateData {
    player: Player,
    water: MyWater,
}

pub struct GameState {
    data_optional: Option<GameStateData>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            data_optional: None,
        }
    }
}

#[async_trait]
impl State<TransitionData, SharedData> for GameState {
    fn on_enter(&mut self, mut payload: StateManagerPayload<SharedData>) {
        let shared_data = &mut payload.shared_data;
        let player_texture = shared_data
            .texture_resources_optional
            .as_ref()
            .unwrap()
            .player;
        let player_pos = vec2(GAME_SIZE.x * 0.5f32, GAME_SIZE.y * 0.5f32);
        let player = Player::new(player_pos, player_texture);
        let water_normal = shared_data
            .raw_image_resources_optional
            .as_mut()
            .unwrap()
            .water_normal
            .take()
            .unwrap();
        self.data_optional = Some(GameStateData {
            player,
            water: MyWater::new(
                water_normal,
                payload.current_rendertarget.texture,
                vec2(GAME_SIZE.x * 7f32, GAME_SIZE.y * 0.5f32),
                vec2(GAME_SIZE.x * -4f32, GAME_SIZE.y * 0.7f32 + 10f32),
            ),
        });
    }

    async fn on_update(
        &mut self,
        _delta_time: f32,
        payload: &mut StateManagerPayload<SharedData>,
    ) -> Option<StateManagerCommand<TransitionData, SharedData>> {
        let shared_data = &mut payload.shared_data;
        let dt = get_frame_time();
        if let Some(game_data) = &mut self.data_optional {
            game_data.player.update(dt);
            game_data.player.process_input(dt, shared_data);

            let water_speed = shared_data.input.get_fraction(4) * 0.5f32;
            let water_strength = shared_data.input.get_fraction(3) * 0.3f32;
            game_data.water.water.strength = water_strength;
            game_data.water.water.speed = water_speed;
            game_data.water.water.update(dt);
        }
        let mul = 500f32;
        let x = shared_data.input.get_fraction(6) * mul;
        let y = 50f32 + shared_data.input.get_fraction(7) * 130.;
        payload.camera.target = vec2(x, y);

        shared_data.input.flush();
        None
    }
    fn on_draw(&mut self, mut payload: StateManagerPayload<SharedData>) {
        let shared_data = &mut payload.shared_data;
        clear_background(WHITE);
        let textures = shared_data.texture_resources_optional.as_ref().unwrap();

        draw_texture_ex(
            textures.scenery,
            -300f32, //x,
            -87f32,  //y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    textures.scenery.width() * 1.5f32,
                    textures.scenery.height() * 1.5f32,
                )),
                ..Default::default()
            },
        );
        if let Some(game_data) = &mut self.data_optional {
            game_data.player.draw();
            game_data.water.water.draw(payload.camera);
        }
    }
}

impl BootState {
    pub fn new(into_state: Box<dyn State<TransitionData, SharedData>>) -> Self {
        Self {
            into_state: Some(into_state),
            texture_resource_builder: ResourceBuilder::<
                TextureIdentifier,
                TextureResources,
                Texture2D,
                DefaultFactory,
            >::new(
                [
                    (
                        TextureIdentifier::Player,
                        "resources/textures/CartoonDetective.png",
                    ),
                    (
                        TextureIdentifier::Scenery,
                        "resources/textures/magic_cliffs_preview.png",
                    ),
                ]
                .into(),
            ),
            raw_image_resource_builder: ResourceBuilder::<
                RawImageResourceIdentifier,
                RawImageResources,
                Image,
                DefaultFactory,
            >::new(
                [(
                    RawImageResourceIdentifier::WaterNormal,
                    "resources/textures/water_normal.png",
                )]
                .into(),
            ),
            raw_sound_resource_builder: ResourceBuilder::<
                SoundIdentifier,
                SoundResources,
                Sound,
                DefaultFactory,
            >::new(
                [
                    (SoundIdentifier::Jump, "resources/sounds/jump.wav"),
                    (SoundIdentifier::Jammed, "resources/sounds/jammed.wav"),
                    (SoundIdentifier::Shoot, "resources/sounds/shoot.wav"),
                    (
                        SoundIdentifier::LatchClose,
                        "resources/sounds/latch_close.wav",
                    ),
                    (
                        SoundIdentifier::LatchOpen,
                        "resources/sounds/latch_open.wav",
                    ),
                    (SoundIdentifier::Crouch, "resources/sounds/crouch.wav"),
                    (SoundIdentifier::Uncrouch, "resources/sounds/Uncrouch.wav"),
                ]
                .into(),
            ),
        }
    }
}

// BootState will load textures asyncronously whilst drawing the procentage process
// when every texture resource is loaded, transition to into_state
pub struct BootState {
    into_state: Option<Box<dyn State<TransitionData, SharedData>>>,
    texture_resource_builder:
        ResourceBuilder<TextureIdentifier, TextureResources, Texture2D, DefaultFactory>,
    raw_image_resource_builder:
        ResourceBuilder<RawImageResourceIdentifier, RawImageResources, Image, DefaultFactory>,
    raw_sound_resource_builder:
        ResourceBuilder<SoundIdentifier, SoundResources, Sound, DefaultFactory>,
}

#[async_trait]
impl State<TransitionData, SharedData> for BootState {
    fn on_enter(&mut self, payload: StateManagerPayload<SharedData>) {
        payload.shared_data.input.connect();
    }

    async fn on_update(
        &mut self,
        _delta_time: f32,
        payload: &mut StateManagerPayload<SharedData>,
    ) -> Option<StateManagerCommand<TransitionData, SharedData>> {
        // load all textures
        let shared_data = &mut payload.shared_data;
        let is_done_loading = self.texture_resource_builder.load_next().await;
        let is_done_loading_2 = self.raw_image_resource_builder.load_next().await;
        let is_done_loading_3 = self.raw_sound_resource_builder.load_next().await;
        if !is_done_loading || !is_done_loading_2 || !is_done_loading_3 {
            return None;
        }
        shared_data.texture_resources_optional = Some(self.texture_resource_builder.build());
        shared_data.raw_image_resources_optional = Some(self.raw_image_resource_builder.build());
        shared_data.sound_resources_optional = Some(self.raw_sound_resource_builder.build().into());
        // unwrap should be safe
        let into_state = self.into_state.take().unwrap();
        return Some(StateManagerCommand::ChangeStateEx(
            into_state,
            TransitionTime(0.3),
            TransitionData::Slide,
        ));
    }
    fn on_draw(&mut self, _payload: StateManagerPayload<SharedData>) {
        clear_background(BLACK);
        draw_text(
            format!(
                "BOOTING UP... {:.0}%",
                self.texture_resource_builder.progress() * 100f32
            )
            .as_str(),
            GAME_SIZE.x * 0.5f32 - 140f32,
            GAME_SIZE.y * 0.5f32,
            40f32,
            WHITE,
        );
    }
}

#[macroquad::main("states")]
async fn main() {
    let render_target_game = render_target(GAME_SIZE.x as u32, GAME_SIZE.y as u32);
    render_target_game.texture.set_filter(FilterMode::Nearest);

    let camera2d = Camera2D {
        zoom: vec2(1. / GAME_SIZE.x * 2., 1. / GAME_SIZE.y * 2.),
        target: vec2(
            (GAME_SIZE.x * 0.5f32).floor(),
            (GAME_SIZE.y * 0.5f32).floor(),
        ),
        render_target: Some(render_target_game),
        ..Default::default()
    };

    let loadingstate_menu = Box::new(GameState::new());
    let boot_state = Box::new(BootState::new(loadingstate_menu));
    let size = RenderTargetSize {
        width: GAME_SIZE.x as u32,
        height: GAME_SIZE.y as u32,
    };
    let transition_tex_slide: Texture2D =
        load_texture("resources/textures/transitions/transition_slide.png")
            .await
            .unwrap();
    let shared_data = SharedData {
        texture_resources_optional: None,
        raw_image_resources_optional: None,
        sound_resources_optional: None,
        input: Input::new(),
    };

    let mut transition_texture_map = HashMap::new();
    transition_texture_map.insert(TransitionData::Slide, transition_tex_slide);
    let mut state_manager: StateManager<TransitionData, SharedData> = StateManager::new(
        boot_state,
        size,
        camera2d,
        shared_data,
        transition_texture_map,
    );

    loop {
        state_manager.update(get_frame_time()).await;
        state_manager.draw();
        next_frame().await
    }
}
