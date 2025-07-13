use raylib::prelude::*;

use crate::gameobject;

pub struct Ball {
    pub radius: f32,
    pub pos: Vector2,
    pub delta_speed: Vector2,
    pub color: Color
}

impl gameobject::Gameobject for Ball {
    fn update(&mut self, handle: &RaylibHandle) {
        self.pos.x += self.delta_speed.x * handle.get_frame_time();
        self.pos.y += self.delta_speed.y * handle.get_frame_time();
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.draw_circle_v(self.pos, self.radius, self.color);
    }
}
