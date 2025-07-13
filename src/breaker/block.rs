use raylib::prelude::*;

use crate::gameobject;

pub struct Block {
    pub body: Rectangle,
    pub color: Color
}

impl gameobject::Gameobject for Block {
    fn update(&mut self, _handle: &RaylibHandle) {
    }
    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.draw_rectangle_pro(self.body, Vector2 {x: 0.0, y: 0.0}, 0.0, self.color);
    }
}
