mod config;
mod game_control;
mod point;
mod user_input;
mod view;
mod world_state;

use crate::game_control::GameControl;
use macroquad::prelude::*;

#[macroquad::main("rustlife")]
async fn main() {
    let mut control = GameControl::new();

    loop {
        control.handle_user_input();
        control.update_state();
        next_frame().await
    }
}
