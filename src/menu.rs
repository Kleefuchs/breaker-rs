use raylib::prelude::*;

use crate::{controllable, gameobject, gamestate, page, text_data};



pub struct Menu<'a, Tfont1: RaylibFont> {
    pub should_start: bool,
    main_text: text_data::TextData<'a, Tfont1>
}

impl<'a, Tfont1: RaylibFont> gameobject::Gameobject for Menu<'a, Tfont1> {
    fn update(&mut self, handle: &raylib::RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut raylib::prelude::RaylibTextureMode<T>) {
        texture_mode.clear_background(Color::BLACK);
        self.main_text.draw(texture_mode);
    }
}

impl<'a, Tfont1: RaylibFont> controllable::Controllable for Menu<'a, Tfont1> {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]) {
        assert!(keys.len()==1);
        if handle.is_key_pressed(keys[0]) {
            self.should_start = true;
        }
    }
}

impl<'a, Tfont1: RaylibFont> page::Page for Menu<'a, Tfont1> {
    fn get_current_state(&self) -> crate::gamestate::Gamestate {
        if self.should_start {
            return gamestate::Gamestate::Init;
        }
        gamestate::Gamestate::Menu
    }
}

impl<'a, Tfont1: RaylibFont> Menu<'a, Tfont1> {
    pub fn new(main_text: text_data::TextData<'a, Tfont1>) -> Menu<'a, Tfont1> {
        Menu {
            should_start: false,
            main_text
        }
    }
}
