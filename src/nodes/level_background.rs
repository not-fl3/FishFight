use macroquad::{
    experimental::{
        collections::storage,
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

pub struct LevelBackground {}

impl LevelBackground {
    pub fn new() -> LevelBackground {
        LevelBackground {}
    }
}

fn parallax(texture: Texture2D, depth: f32, camera_pos: Vec2) -> Rect {
    let w = texture.width();
    let h = texture.height();

    let dest_rect = Rect::new(0., 0., w, h);
    let parallax_w = w as f32 * 0.5;

    let mut dest_rect2 = Rect::new(
        -parallax_w,
        -parallax_w,
        w + parallax_w * 2.,
        h + parallax_w * 2.,
    );

    let parallax_x = camera_pos.x / dest_rect.w - 0.3;
    let parallax_y = camera_pos.y / dest_rect.h * 0.6 - 0.5;

    dest_rect2.x += parallax_w * parallax_x * depth;
    dest_rect2.y += parallax_w * parallax_y * depth;

    dest_rect2
}

impl scene::Node for LevelBackground {
    fn draw(_node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        let pos = scene::camera_pos();

        clear_background(Color::from_rgba(126, 168, 166, 255));

        let dest_rect = parallax(resources.background_03.clone(), 2.0, pos);
        draw_texture_ex(
            &resources.background_03,
            dest_rect.x,
            80.0 + dest_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                ..Default::default()
            },
        );
        let dest_rect = parallax(resources.background_02.clone(), 1.0, pos);
        draw_texture_ex(
            &resources.background_02,
            dest_rect.x,
            120.0 + dest_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                ..Default::default()
            },
        );

        let dest_rect = parallax(resources.background_01.clone(), 0.5, pos);
        draw_texture_ex(
            &resources.background_01,
            dest_rect.x,
            180.0 + dest_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_rect.w, dest_rect.h)),
                ..Default::default()
            },
        );

        let w =
            resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
        let h =
            resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;
        resources
            .tiled_map
            .draw_tiles("main layer", Rect::new(0.0, 0.0, w as _, h as _), None);
    }
}
