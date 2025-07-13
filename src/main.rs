
use raylib::prelude::*;
use crate::{controllable::Controllable, game::Game, gameobject::Gameobject};

mod breaker;
mod gameobject;
mod rl;
mod controllable;
mod game;
mod gamestate;
mod game_over;
mod text_data;

fn create_default_breaker(world_size: Vector2) -> breaker::Breaker {
    breaker::Breaker::new(world_size.x as i32, world_size.y as i32, 2.5, &mut [Color::RED, Color::ORANGE, Color::YELLOW, Color::GREEN, Color::BLUE, Color::PURPLE])
}

fn create_default_game_over<'a, Tfont: RaylibFont>(text: &'a str, world_size: Vector2, font: &'a Tfont, font_size: f32, measured_text: i32) -> game_over::GameOver<'a, Tfont> {
    game_over::GameOver::new(
        text_data::TextData {
            font,
            text,
            position: Vector2 {x: world_size.x / 2.0, y: world_size.y / 3.0},
            origin: Vector2 {x: measured_text as f32 / 2.0, y: 0.0},
            rotation: 0.0,
            font_size,
            spacing: 1.0,
            tint: Color::WHITE,
        }
    )

}

fn main() {
    let world_size: Vector2 = Vector2 {x: 640.0, y: 360.0};
    let mut rl = rl::Rl::from(raylib::init().size(1280, 720).title("Breaker").build());
    let mut render_texture: RenderTexture2D = rl.handle.load_render_texture(&rl.thread, world_size.x as u32, world_size.y as u32).unwrap();
    let mut breaker: breaker::Breaker = create_default_breaker(world_size);
    let font_default = rl.handle.get_font_default();
    let mut game_over: game_over::GameOver<WeakFont> = create_default_game_over("Game Over", world_size, &font_default, 30.0, rl.handle.measure_text("Game Over", 30));
    let mut gamestate: gamestate::Gamestate = gamestate::Gamestate::Running;
    while !rl.handle.window_should_close() {
        match gamestate {
            gamestate::Gamestate::Running => {
                breaker.control(&rl.handle, &mut [KeyboardKey::KEY_A, KeyboardKey::KEY_D]);
                breaker.update(&rl.handle);
                gamestate = breaker.get_current_state();
                {
                    let mut texture_mode = rl.handle.begin_texture_mode(&rl.thread, &mut render_texture);
                    breaker.draw(&mut texture_mode);
                }
            },
            gamestate::Gamestate::Paused => {

            },
            gamestate::Gamestate::GameOver => {
                game_over.control(&rl.handle, &mut [KeyboardKey::KEY_SPACE]);
                game_over.update(&rl.handle);
                gamestate = game_over.get_current_state();
                let mut texture_mode = rl.handle.begin_texture_mode(&rl.thread, &mut render_texture);
                game_over.draw(&mut texture_mode);
            },
            gamestate::Gamestate::Init => {
                breaker = create_default_breaker(world_size);
                gamestate = gamestate::Gamestate::Running;
                game_over = create_default_game_over("Game Over", world_size, &font_default, 30.0, rl.handle.measure_text("Game Over", 30));
            },
            gamestate::Gamestate::Menu => {

            }
        }
                let screen_width: i32 = rl.handle.get_screen_width();
                let screen_height: i32 = rl.handle.get_screen_height();
                let mut draw_handle: RaylibDrawHandle = rl.handle.begin_drawing(&rl.thread);
                draw_handle.clear_background(Color::BLACK);
                draw_handle.draw_texture_pro(&render_texture, Rectangle {x: 0.0, y: 0.0, width: world_size.x, height: -world_size.y}, Rectangle {x: 0.0, y: 0.0, width: screen_width as f32, height: screen_height as f32}, Vector2 {x: 0.0, y: 0.0}, 0.0, Color::WHITE);
    }
}
