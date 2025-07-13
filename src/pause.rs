use raylib::prelude::*;

use crate::{controllable, gameobject, gamestate, page, text_data};

pub struct Pause<'a, Tfont: RaylibFont> {
    pub should_unpause: bool,
    text_data: text_data::TextData<'a, Tfont>
}

impl<'a, Tfont: RaylibFont> controllable::Controllable for Pause<'a, Tfont> {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]) {
        assert!(keys.len()==1);
        if handle.is_key_pressed(keys[0]) {
            self.should_unpause = true;
        }
    }
}

impl<'a, Tfont: RaylibFont> gameobject::Gameobject for Pause<'a, Tfont> {
    fn update(&mut self, _handle: &RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut RaylibTextureMode<T>) {
        self.text_data.draw(texture_mode);
    }
}

impl<'a, Tfont: RaylibFont> page::Page for Pause<'a, Tfont> {
    fn get_current_state(&self) -> crate::gamestate::Gamestate {
        if self.should_unpause {
            return gamestate::Gamestate::Resume;
        }
        gamestate::Gamestate::Paused
    }
}

impl<'a, Tfont: RaylibFont> Pause<'a, Tfont> {
    pub fn new(text_data: text_data::TextData<'a, Tfont>) -> Pause<'a, Tfont> {
        Pause {
            should_unpause: false,
            text_data
        }
    }
}
