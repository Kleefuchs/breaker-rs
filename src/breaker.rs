use raylib::prelude::*;
use crate::{controllable,  gameobject};
mod platform;
mod block;
mod ball;
mod block_data;
use crate::page;
use crate::gamestate;

pub struct Breaker {
    world_width: i32,
    world_height: i32,
    platform: platform::Platform,
    block_data: block_data::BlockData,
    ball: ball::Ball,
    platform_delta_x_change_factor: f32,
    pub should_pause: bool,
}

impl Breaker {
    pub fn new(world_width: i32, world_height: i32, platform_delta_x_change_factor: f32, colors: &mut [Color]) -> Breaker {
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
            block_data: block_data::BlockData::new(world_width, world_height, 3, 6, 6, 1, colors),
            ball: ball::Ball {
                pos: Vector2 {
                    x: (world_width / 2) as f32,
                    y: (world_height / 2) as f32
                },
                radius: 5.0,
                delta_speed: Vector2 {
                    x: 175.0,
                    y: 175.0
                },
                color: Color::WHITE,
            },
            platform_delta_x_change_factor,
            should_pause: false,
        }
    }   
}

impl controllable::Controllable for Breaker {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]) {
        assert!(keys.len()==3);
        if handle.is_key_down(keys[0]) & (self.platform.body.x > 0.0) {
            self.platform.body.x -= 300.0 * handle.get_frame_time();
        }

        if handle.is_key_down(keys[1]) & (self.platform.body.x + self.platform.body.width < self.world_width as f32) {
            self.platform.body.x += 300.0 * handle.get_frame_time();
        }

        if handle.is_key_pressed(keys[2]) {
            self.should_pause = true;
        }
    }
}

impl page::Page for Breaker {
    fn get_current_state(&self) -> gamestate::Gamestate {
        if self.ball.pos.y - self.ball.radius > self.world_height as f32 {
            return gamestate::Gamestate::GameOver;
        }

        if self.should_pause {
            return gamestate::Gamestate::Paused;
        }

        gamestate::Gamestate::Running
    }
}

impl gameobject::Gameobject for Breaker {
    fn update(&mut self, handle: &RaylibHandle) {
        self.ball.update(handle);
        if self.platform.body.check_collision_circle_rec(self.ball.pos, self.ball.radius) && (self.ball.delta_speed.y > 0.0) {
            self.ball.delta_speed.y *= -1.0;
            self.ball.delta_speed.x = (self.ball.pos.x - (self.platform.body.x + self.platform.body.width / 2.0)) * self.platform_delta_x_change_factor;
        }

        if self.ball.pos.x + self.ball.radius >= self.world_width as f32 && (self.ball.delta_speed.x > 0.0) {
            self.ball.delta_speed.x *= -1.0;
        }

        if self.ball.pos.x - self.ball.radius <= 0.0 && (self.ball.delta_speed.x < 0.0) {
            self.ball.delta_speed.x *= -1.0;
        }

        if self.ball.pos.y - self.ball.radius <= 0.0 && (self.ball.delta_speed.y < 0.0) {
            self.ball.delta_speed.y *= -1.0;
        }

        if self.block_data.block_area.check_collision_circle_rec(self.ball.pos, self.ball.radius) {
            for block_index in 0..self.block_data.blocks.len() {
                let block: &block::Block = &self.block_data.blocks[block_index];
                if block.body.check_collision_circle_rec(self.ball.pos, self.ball.radius) {
                    if (self.ball.pos.x + self.ball.radius >= block.body.x) && (self.ball.pos.x - self.ball.radius <= block.body.x + block.body.width) {
                        self.ball.delta_speed.y *= -1.0;
                    } else {
                        self.ball.delta_speed.x *= -1.0;
                    }
                    self.block_data.blocks.remove(block_index);
                    break;
                }
            }
        }

    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.clear_background(Color::BLACK);
        self.platform.draw(texture_mode);
        self.block_data.draw(texture_mode);
        self.ball.draw(texture_mode);
        texture_mode.draw_fps(10, 10);
    }
}
