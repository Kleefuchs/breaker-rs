use raylib::prelude::*;

pub trait Gameobject {
    fn update(&mut self);
    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>);
}
