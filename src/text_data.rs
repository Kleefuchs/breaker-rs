use raylib::prelude::*;

use crate::gameobject;

pub struct TextData<'a, Tfont: RaylibFont> {
    pub font: &'a Tfont,
    pub text: &'a str,
    pub position: Vector2,
    pub origin: Vector2,
    pub rotation: f32,
    pub font_size: f32,
    pub spacing: f32,
    pub tint: Color,
}

impl<Tfont: RaylibFont> gameobject::Gameobject for TextData<'_, Tfont> {
    fn update(&mut self, _handle: &RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        texture_mode.draw_text_pro(self.font, self.text, self.position, self.origin, self.rotation, self.font_size, self.spacing, self.tint);
    }
}
