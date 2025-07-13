use raylib::prelude::*;
use crate::gameobject;

pub struct Platform {
    pub body: Rectangle,
    pub color: Color,
}


impl gameobject::Gameobject for Platform {
    fn update(&mut self, _handle: &RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.draw_rectangle_rec(self.body, self.color);
    }
}

