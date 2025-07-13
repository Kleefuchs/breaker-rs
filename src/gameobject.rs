use raylib::prelude::*;

pub trait Gameobject {
    fn update(&mut self, handle: &RaylibHandle);
    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>);
}
