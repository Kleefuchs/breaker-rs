use crate::gameobject;
use crate::gamestate;

pub trait Page: gameobject::Gameobject {
    fn get_current_state(&self) -> gamestate::Gamestate;
}
