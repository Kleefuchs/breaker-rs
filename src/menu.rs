use raylib::prelude::*;

use crate::{gameobject, gamestate, page};



pub struct Menu {
}

impl gameobject::Gameobject for Menu {
    fn update(&mut self, handle: &raylib::RaylibHandle) {
    }

    fn draw<T>(&self, texture_mode: &mut raylib::prelude::RaylibTextureMode<T>) {
        texture_mode.clear_background(Color::BLACK);
    }
}

impl page::Page for Menu {
    fn get_current_state(&self) -> crate::gamestate::Gamestate {
        gamestate::Gamestate::Menu
    }
}
