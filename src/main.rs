
use raylib::prelude::*;
use crate::{controllable::Controllable, gameobject::Gameobject};

mod breaker;
mod gameobject;
mod rl;
mod controllable;

fn main() {
    let world_size: Vector2 = Vector2 {x: 640.0, y: 360.0};
    let mut rl = rl::Rl::from(raylib::init().size(1280, 720).title("Breaker").build());
    let mut render_texture: RenderTexture2D = rl.handle.load_render_texture(&rl.thread, world_size.x as u32, world_size.y as u32).unwrap();
    let mut game: breaker::Breaker = breaker::Breaker::new(world_size.x as i32, world_size.y as i32);
    while !rl.handle.window_should_close() {
        game.control(&rl.handle, &mut [KeyboardKey::KEY_A, KeyboardKey::KEY_D]);
        game.update();
        {
            let mut texture_mode = rl.handle.begin_texture_mode(&rl.thread, &mut render_texture);
            game.draw(&mut texture_mode);
        }
        let screen_width: i32 = rl.handle.get_screen_width();
        let screen_height: i32 = rl.handle.get_screen_height();
        let mut draw_handle: RaylibDrawHandle = rl.handle.begin_drawing(&rl.thread);
        draw_handle.clear_background(Color::BLACK);
        draw_handle.draw_texture_pro(&render_texture, Rectangle {x: 0.0, y: 0.0, width: world_size.x, height: -world_size.y}, Rectangle {x: 0.0, y: 0.0, width: screen_width as f32, height: screen_height as f32}, Vector2 {x: 0.0, y: 0.0}, 0.0, Color::WHITE);
    }
}
