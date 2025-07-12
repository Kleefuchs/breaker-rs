use raylib::prelude::*;

use crate::gameobject;

pub struct Ball {
    radius: f32,
    pos: Vector2
}

impl gameobject::Gameobject for Ball {
    fn update(&mut self) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {

    }
}
