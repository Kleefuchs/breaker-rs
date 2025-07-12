use raylib::prelude::*;

pub trait Controllable {
    fn control(&mut self, handle: &RaylibHandle, keys: &mut [KeyboardKey]);
}
