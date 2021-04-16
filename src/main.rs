use crate::input::*;
use async_trait::async_trait;
use macroquad::prelude::*;
use macroquad_tantan_toolbox::animation::*;
use macroquad_tantan_toolbox::resources::*;
use macroquad_tantan_toolbox::states::*;
use macroquad_tantan_toolbox::water::*;
use std::collections::HashMap;

mod input;

const GAME_SIZE: Vec2 = Vec2 {
    x: 514f32,
    y: 256f32,
};

const GRAVITY: f32 = 30f32;
const MOVE_SPEED: f32 = 200f32;
const MAX_JUMP_STRENGTH: f32 = 12f32;

pub struct Player {
    animation: AnimationInstance<PlayerAnimationIdentifier>,
    pos: Vec2,
    y_vel: f32,
    jump_strength: f32,
    is_grounded: bool,
}

impl Player {
    pub fn new(pos: Vec2, animation: AnimationInstance<PlayerAnimationIdentifier>) -> Self {
        Self {
            animation,
            pos,
            y_vel: 0f32,
            jump_strength: 0.05f32,
            is_grounded: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.pos.y < GAME_SIZE.y * 0.4f32 {
            self.y_vel += dt * GRAVITY;
        }
        self.pos.y += self.y_vel;
        if self.pos.y >= GAME_SIZE.y * 0.4f32 {
            self.is_grounded = true;
            self.y_vel = 0f32;
            self.pos.y = GAME_SIZE.y * 0.4f32;
        } else {
            self.is_grounded = false;
        }
        self.animation.update(dt);
    }

    pub fn jump(&mut self) {
        if self.is_grounded {
            self.y_vel = -self.jump_strength * MAX_JUMP_STRENGTH;
        }
    }

    pub fn set_jump_strength(&mut self, v: f32) {
        self.jump_strength = v;
    }


    pub fn draw(&mut self) {
        self.animation.draw(&self.pos);
    }
}

pub struct GameStateData {
    player: Player,
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
    fn on_enter(&mut self, shared_data: &mut SharedData) {
        let player_texture = shared_data
            .texture_resources_optional
            .as_ref()
            .unwrap()
            .player;
        let mut player_animation = AnimationInstance::<PlayerAnimationIdentifier>::new(
            10f32,
            8f32,
            player_texture,
            PlayerAnimationIdentifier::Idle,
        );
        player_animation.add_animation(0, 3, None, 10f32, PlayerAnimationIdentifier::Idle);
        player_animation.add_animation(10, 16, None, 10f32, PlayerAnimationIdentifier::Run);
        player_animation.add_animation(20, 24, None, 10f32, PlayerAnimationIdentifier::Shoot);
        player_animation.add_animation(36, 46, None, 10f32, PlayerAnimationIdentifier::Reload);
        let player_pos = vec2(GAME_SIZE.x * 0.5f32, GAME_SIZE.y * 0.5f32);
        let player = Player::new(player_pos, player_animation);
        self.data_optional = Some(GameStateData { player });
    }

    async fn on_update(
        &mut self,
        _delta_time: f32,
        shared_data: &mut SharedData,
    ) -> Option<StateManagerCommand<TransitionData, SharedData>> {
        let dt = get_frame_time();
        if let Some(game_data) = &mut self.data_optional {
            game_data.player.update(dt);

            let mut next_player_anim_optional = None;
            if shared_data.input.is_button_held(41) {
                game_data.player.pos.x -= MOVE_SPEED * dt;
                next_player_anim_optional = Some(PlayerAnimationIdentifier::Run);
            } else if shared_data.input.is_button_held(45) {
                game_data.player.pos.x += MOVE_SPEED * dt;
                next_player_anim_optional = Some(PlayerAnimationIdentifier::Run);
            } else {
                next_player_anim_optional = Some(PlayerAnimationIdentifier::Idle);
            }

            if shared_data.input.is_button_held(64) {
                game_data.player.jump();
            }

            game_data.player.set_jump_strength(shared_data.input.get_fraction(0));

            if let Some(wanted_anim) = next_player_anim_optional {
                if game_data.player.animation.current_animation != wanted_anim {
                    game_data.player.animation.play_animation(wanted_anim);
                }
            }
        }
        shared_data.input.flush();
        None
    }
    fn on_draw(&mut self, shared_data: &mut SharedData) {
        clear_background(WHITE);
        let textures = shared_data.texture_resources_optional.as_ref().unwrap();
        let mul = 500f32;
        let x = -shared_data.input.get_fraction(6)* mul;
        let y = -shared_data.input.get_fraction(7) * mul;
        draw_texture_ex(textures.scenery, x, y, WHITE, DrawTextureParams {dest_size: Some(vec2(textures.scenery.width()*1.5f32, textures.scenery.height()*1.5f32)), ..Default::default()});
        if let Some(game_data) = &mut self.data_optional {
            game_data.player.draw();
        }
    }
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum TextureIdentifier {
    Player,
    Scenery,
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum PlayerAnimationIdentifier {
    Idle,
    Run,
    Shoot,
    Reload,
}

pub struct TextureResources {
    player: Texture2D,
    scenery: Texture2D,
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

// BootState will load textures asyncronously whilst drawing the procentage process
// when every texture resource is loaded, transition to into_state
pub struct BootState {
    into_state: Option<Box<dyn State<TransitionData, SharedData>>>,
    texture_resource_builder:
        ResourceBuilder<TextureIdentifier, TextureResources, Texture2D, DefaultFactory>,
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
                [(
                    TextureIdentifier::Player,
                    "resources/textures/CartoonDetective.png",
                ),(
                    TextureIdentifier::Scenery,
                    "resources/textures/magic_cliffs_preview.png",
                )
                ]
                .into(),
            ),
        }
    }
}

#[async_trait]
impl State<TransitionData, SharedData> for BootState {
    fn on_enter(&mut self, shared_data: &mut SharedData) {
        shared_data.input.connect();
    }

    async fn on_update(
        &mut self,
        _delta_time: f32,
        shared_data: &mut SharedData,
    ) -> Option<StateManagerCommand<TransitionData, SharedData>> {
        // load all textures
        let is_done_loading = self.texture_resource_builder.load_next().await;
        if !is_done_loading {
            return None;
        }
        shared_data.texture_resources_optional = Some(self.texture_resource_builder.build());
        // unwrap should be safe
        let into_state = self.into_state.take().unwrap();
        return Some(StateManagerCommand::ChangeStateEx(
            into_state,
            TransitionTime(0.3),
            TransitionData::Slide,
        ));
    }
    fn on_draw(&mut self, _shared_data: &mut SharedData) {
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
pub struct SharedData {
    texture_resources_optional: Option<TextureResources>,
    input: Input,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum TransitionData {
    Slide,
}

impl Default for TransitionData {
    fn default() -> Self {
        TransitionData::Slide
    }
}

#[macroquad::main("states")]
async fn main() {
    let render_target_game = render_target(GAME_SIZE.x as u32, GAME_SIZE.y as u32);
    set_texture_filter(render_target_game.texture, FilterMode::Nearest);

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
        load_texture("resources/textures/transitions/transition_slide.png").await;
    let shared_data = SharedData {
        texture_resources_optional: None,
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
