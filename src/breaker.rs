use raylib::prelude::*;
use crate::{controllable,  gameobject};
mod platform;
mod block;
mod blocks;
mod ball;

pub struct Breaker {
    world_width: i32,
    world_height: i32,
    platform: platform::Platform,
    blocks: Vec<block::Block>,
}

impl Breaker {
    pub fn new(world_width: i32, world_height: i32) -> Breaker {
        Breaker {
            world_width,
            world_height,
            platform: platform::Platform {
                body: Rectangle {
                    x: (world_width / 2 - 50) as f32,
                    y: (world_height - world_height / 5) as f32,
                    width: 100.0,
                    height: 20.0,
                },
                color: Color::WHITE,
            },
            blocks: blocks::generate_blocks(world_width, world_height, 3, 5, 4, 1),
        }
    }   
}

impl controllable::Controllable for Breaker {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]) {
        assert!(keys.len()==2);
        if handle.is_key_down(keys[0]) & (self.platform.body.x > 0.0) {
            self.platform.body.x -= 300.0 * handle.get_frame_time();
        }
        if handle.is_key_down(keys[1]) & (self.platform.body.x + self.platform.body.width < self.world_width as f32) {
            self.platform.body.x += 300.0 * handle.get_frame_time();
        }
    }
}

impl gameobject::Gameobject for Breaker {
    fn update(&mut self) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.clear_background(Color::BLACK);
        self.platform.draw(texture_mode);
        for block in &self.blocks {
            block.draw(texture_mode);
        }
        texture_mode.draw_fps(10, 10);
    }
}
