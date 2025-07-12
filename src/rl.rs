use raylib::prelude::*;

pub struct Rl {
    pub handle: RaylibHandle,
    pub thread: RaylibThread
}

impl From<(raylib::RaylibHandle, raylib::RaylibThread)> for Rl {
    fn from(rl_tuple: (raylib::RaylibHandle, raylib::RaylibThread)) -> Rl {
        Rl {
            handle: rl_tuple.0,
            thread: rl_tuple.1
        }
    }
}
