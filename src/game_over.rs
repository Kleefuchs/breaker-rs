use raylib::prelude::*;

use crate::{
    controllable, page, gameobject, gamestate,
    text_data::{TextData},
};

pub struct GameOver<'a, Tfont: RaylibFont> {
    pub return_to_game: bool,
    text_data: TextData<'a, Tfont>,
}

impl<Tfont: RaylibFont> controllable::Controllable for GameOver<'_, Tfont> {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]) {
        assert!(keys.len() == 1);
        if handle.is_key_pressed(keys[0]) {
            self.return_to_game = true;
        }
    }
}

impl<Tfont: RaylibFont> gameobject::Gameobject for GameOver<'_, Tfont> {
    fn update(&mut self, _handle: &raylib::RaylibHandle) {}

    fn draw<T>(&self, texture_mode: &mut raylib::prelude::RaylibTextureMode<T>) {
        texture_mode.clear_background(Color::BLACK);
        self.text_data.draw(texture_mode);
    }
}

impl<Tfont: RaylibFont> page::Page for GameOver<'_, Tfont> {
    fn get_current_state(&self) -> gamestate::Gamestate {
        if self.return_to_game {
            return gamestate::Gamestate::Init;
        }
        gamestate::Gamestate::GameOver
    }
}

impl<'a, Tfont: RaylibFont> GameOver<'a, Tfont> {
    pub fn new(text_data: TextData<'a, Tfont>) -> GameOver<'a, Tfont> {
        GameOver {
            return_to_game: false,
            text_data,
        }
    }
}
