use crate::world_state::WorldState;
use macroquad::prelude::*;

pub struct WorldView {
    pub camera: Camera2D,
}

pub const POINT_SIZE: f32 = 10.0;
const LINE_THICKNESS: f32 = 0.5;

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

    pub fn draw_grid(&self) {
        let top_left = self.camera.screen_to_world(vec2(0.0, 0.0));
        let bottom_right = self.camera.screen_to_world(vec2(screen_width(), screen_height()));

        let mut x = (top_left.x / POINT_SIZE).floor() * POINT_SIZE;
        while x <= bottom_right.x {
            draw_line(x, top_left.y, x, bottom_right.y, LINE_THICKNESS, WHITE);
            x += POINT_SIZE;
        }

        let mut y = (bottom_right.y / POINT_SIZE).floor() * POINT_SIZE;
        while y <= top_left.y {
            draw_line(top_left.x, y, bottom_right.x, y, LINE_THICKNESS, WHITE);
            y += POINT_SIZE
        }
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
