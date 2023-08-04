use crate::world_state::WorldState;
use macroquad::prelude::*;

pub struct WorldView {
    pub camera: Camera2D,
}

pub const POINT_SIZE: f32 = 10.0;

impl WorldView {
    pub fn new() -> Self {
        Self {
            camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, 800.0, 600.0)),
        }
    }
    pub fn draw(&self, world: &WorldState) {
        world.field.iter().for_each(|point| {
            draw_rectangle(
                point.x as f32 * POINT_SIZE,
                point.y as f32 * POINT_SIZE,
                POINT_SIZE,
                POINT_SIZE,
                WHITE,
            )
        });
    }
    pub fn update_camera(&mut self) {
        clear_background(BLACK);

        if is_key_down(KeyCode::Left) {
            self.camera.target.x -= 5.0;
        }
        if is_key_down(KeyCode::Right) {
            self.camera.target.x += 5.0;
        }
        if is_key_down(KeyCode::Up) {
            self.camera.target.y += 5.0;
        }
        if is_key_down(KeyCode::Down) {
            self.camera.target.y -= 5.0;
        }

        if is_key_down(KeyCode::KpAdd) {
            self.camera.zoom.x += self.camera.zoom.x * 0.01;
            self.camera.zoom.y += self.camera.zoom.y * 0.01;
        }
        if is_key_down(KeyCode::KpSubtract) {
            self.camera.zoom.x -= self.camera.zoom.x * 0.01;
            self.camera.zoom.y -= self.camera.zoom.y * 0.01;
        }
        set_camera(&self.camera)
    }
}
