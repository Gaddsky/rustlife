use crate::config::Config;
use crate::user_input::{user_starts_game, user_toggled_point};
use crate::world_state::WorldState;
use macroquad::prelude::get_frame_time;
use crate::view::WorldView;


pub struct GameControl {
    world: WorldState,
    config: Config,
    view: WorldView,
    delta_time: f32,
    started: bool,
}

impl GameControl {
    pub fn new() -> Self {
        Self {
            world: WorldState::new(),
            config: Config::new(),
            view: WorldView::new(),
            delta_time: 0.0,
            started: false,
        }
    }
    pub fn handle_user_input(&mut self) {
        if !self.started {
            if let Some(p) = user_toggled_point(&self.view) {
                self.world.toggle_point(p)
            }
            self.started = user_starts_game()
        }
    }

    pub fn update_state(&mut self) {
        if self.started {
            self.delta_time += get_frame_time();
            if self.delta_time > 1.0 / self.config.fps as f32 {
                self.world.next_step();
                self.delta_time = 0.0;
            }
        }
        self.view.update_camera();
        self.view.draw(&self.world);
    }
}
