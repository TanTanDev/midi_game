use crate::constants::*;
use crate::gun::*;
use crate::input::*;
use crate::resources::*;
use macroquad::audio::*;
use macroquad::prelude::*;
use macroquad_tantan_toolbox::animation::*;

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum PlayerAnimationIdentifier {
    Idle,
    Run,
    Shoot,
    HatchOpen,
    HatchClose,
    Jammed,
    CrouchIdle,
    CrouchRun,
    CrouchHatchOpen,
    CrouchHatchClose,
    CrouchJammed,
    CrouchShoot,
}

#[derive(Hash, Eq, Clone, Debug, Copy, PartialEq)]
pub enum PlayerAnimationNoStanceIdentifier {
    Idle,
    Run,
    Shoot,
    HatchOpen,
    HatchClose,
    Jammed,
}

impl PlayerAnimationNoStanceIdentifier {
    pub fn to_animation(self, is_crouched: bool) -> PlayerAnimationIdentifier {
        use PlayerAnimationNoStanceIdentifier::*;
        match is_crouched {
            true => match self {
                Idle => PlayerAnimationIdentifier::CrouchIdle,
                Run => PlayerAnimationIdentifier::CrouchRun,
                Shoot => PlayerAnimationIdentifier::CrouchShoot,
                HatchOpen => PlayerAnimationIdentifier::CrouchHatchOpen,
                HatchClose => PlayerAnimationIdentifier::CrouchHatchClose,
                Jammed => PlayerAnimationIdentifier::CrouchJammed,
            },
            false => match self {
                Idle => PlayerAnimationIdentifier::Idle,
                Run => PlayerAnimationIdentifier::Run,
                Shoot => PlayerAnimationIdentifier::Shoot,
                HatchOpen => PlayerAnimationIdentifier::HatchOpen,
                HatchClose => PlayerAnimationIdentifier::HatchClose,
                Jammed => PlayerAnimationIdentifier::Jammed,
            },
        }
    }
}

pub struct Player {
    pub animation: AnimationInstance<PlayerAnimationIdentifier>,
    pub pos: Vec2,
    pub y_vel: f32,
    pub jump_strength: f32,
    pub is_grounded: bool,
    pub gun: Gun,
    pub is_crouching: bool,
    pub is_facing_right: bool,
}

impl Player {
    pub fn new(pos: Vec2, player_texture: Texture2D) -> Self {
        let mut animation = AnimationInstance::<PlayerAnimationIdentifier>::new(
            10f32,
            13f32,
            player_texture,
            PlayerAnimationIdentifier::Idle,
        );
        animation.add_animation(0, 3, None, 10f32, PlayerAnimationIdentifier::Idle);
        animation.add_animation(10, 16, None, 10f32, PlayerAnimationIdentifier::Run);
        animation.add_animation(20, 24, None, 10f32, PlayerAnimationIdentifier::Shoot);
        animation.add_animation(36, 46, None, 20f32, PlayerAnimationIdentifier::HatchOpen);
        animation.add_animation(50, 55, None, 10f32, PlayerAnimationIdentifier::Jammed);
        animation.add_animation(60, 64, None, 10f32, PlayerAnimationIdentifier::HatchClose);
        animation.add_animation(70, 73, None, 10f32, PlayerAnimationIdentifier::CrouchIdle);
        animation.add_animation(80, 83, None, 10f32, PlayerAnimationIdentifier::CrouchRun);
        animation.add_animation(
            90,
            94,
            None,
            10f32,
            PlayerAnimationIdentifier::CrouchHatchOpen,
        );
        animation.add_animation(
            100,
            103,
            None,
            10f32,
            PlayerAnimationIdentifier::CrouchHatchClose,
        );
        animation.add_animation(
            110,
            113,
            None,
            10f32,
            PlayerAnimationIdentifier::CrouchJammed,
        );
        animation.add_animation(
            120,
            124,
            None,
            10f32,
            PlayerAnimationIdentifier::CrouchShoot,
        );
        Self {
            animation,
            pos,
            y_vel: 0f32,
            jump_strength: 0.05f32,
            is_grounded: false,
            gun: Gun::new(),
            is_crouching: false,
            is_facing_right: false,
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
        self.animation.draw(&self.pos, !self.is_facing_right);
    }

    pub fn process_input(&mut self, dt: f32, shared_data: &mut SharedData) {
        let mut next_player_anim_optional = None;
        if shared_data.input.is_button_held(41) {
            self.pos.x -= MOVE_SPEED * dt;
            self.is_facing_right = false;
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Run);
        } else if shared_data.input.is_button_held(45) {
            self.is_facing_right = true;
            self.pos.x += MOVE_SPEED * dt;
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Run);
        }

        if shared_data.input.is_button_held(64) {
            self.jump();
        }

        if shared_data.input.is_button_pressed(64) {
            self.jump();
            play_sound_once(shared_data.sound_resources_optional.as_ref().unwrap().jump);
        }

        // try shot gun
        if shared_data.input.is_button_pressed(65) {
            let result = self.gun.try_consume();
            match result {
                Ok(()) => {
                    next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Shoot);
                    play_sound_once(shared_data.sound_resources_optional.as_ref().unwrap().shoot);
                }
                Err(consume_error) => match consume_error {
                    ConsumeError::LatchWasOpen => {
                        next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Jammed);
                        play_sound_once(
                            shared_data
                                .sound_resources_optional
                                .as_ref()
                                .unwrap()
                                .jammed,
                        );
                    }
                    ConsumeError::NotLoaded => {
                        next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Jammed);
                        play_sound_once(
                            shared_data
                                .sound_resources_optional
                                .as_ref()
                                .unwrap()
                                .jammed,
                        );
                    }
                },
            }
        }

        if shared_data
            .input
            .fraction_reached_limit(1, 0.7, SliderLimitCheck::Higher)
        {
            self.gun.set_latch_state(LatchState::Open);
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::HatchOpen);
            play_sound_once(
                shared_data
                    .sound_resources_optional
                    .as_ref()
                    .unwrap()
                    .latch_open,
            );
        }
        if shared_data
            .input
            .fraction_reached_limit(1, 0.3, SliderLimitCheck::Lower)
        {
            self.gun.set_latch_state(LatchState::Closed);
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::HatchClose);
            play_sound_once(
                shared_data
                    .sound_resources_optional
                    .as_ref()
                    .unwrap()
                    .latch_close,
            );
        }

        if shared_data
            .input
            .fraction_reached_limit(2, 0.7, SliderLimitCheck::Higher)
        {
            self.is_crouching = false;
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Idle);
            play_sound_once(
                shared_data
                    .sound_resources_optional
                    .as_ref()
                    .unwrap()
                    .uncrouch,
            );
        }
        if shared_data
            .input
            .fraction_reached_limit(2, 0.3, SliderLimitCheck::Lower)
        {
            self.is_crouching = true;
            next_player_anim_optional = Some(PlayerAnimationNoStanceIdentifier::Idle);
            play_sound_once(
                shared_data
                    .sound_resources_optional
                    .as_ref()
                    .unwrap()
                    .crouch,
            );
        }

        self.set_jump_strength(shared_data.input.get_fraction(0));

        if let Some(wanted_anim_no_stance) = next_player_anim_optional {
            let wanted_anim = wanted_anim_no_stance.to_animation(self.is_crouching);
            if self.animation.current_animation != wanted_anim {
                self.animation.play_animation_then(
                    wanted_anim,
                    PlayerAnimationNoStanceIdentifier::Idle.to_animation(self.is_crouching),
                );
            }
        }
    }
}
