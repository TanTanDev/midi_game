use macroquad::prelude::*;
use macroquad_tantan_toolbox::water::*;

pub struct MyWater {
    pub water: Water,
}
impl MyWater {
    pub fn new(
        texture_water_raw: Image,
        render_target_texture: Texture2D,
        water_size: Vec2,
        pos: Vec2,
    ) -> Self {
        let tex_water_normal = {
            // All of this is so we can sample the texture repeatedly
            use miniquad::{FilterMode, TextureFormat, TextureParams, TextureWrap};
            let ctx = unsafe { get_internal_gl().quad_context };
            let texture_miniquad = miniquad::graphics::Texture::from_data_and_format(
                ctx,
                &texture_water_raw.bytes,
                TextureParams {
                    format: TextureFormat::RGBA8,
                    wrap: TextureWrap::Repeat,
                    filter: FilterMode::Linear,
                    width: texture_water_raw.width as u32,
                    height: texture_water_raw.height as u32,
                },
            );
            Texture2D::from_miniquad_texture(texture_miniquad)
        };

        let water_pos = pos;
        let water_dir = vec2(1.0f32, 0.0f32);
        let water_speed = 0.025f32;
        let water_strength = 0.02f32;
        let mut water = Water::new(
            water_pos,
            water_size,
            render_target_texture,
            tex_water_normal,
            water_dir,
            water_speed,
            water_strength,
            0f32,
        );
        Self { water }
    }
}
