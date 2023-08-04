use crate::point::Point;
use macroquad::prelude::*;
use crate::view::{POINT_SIZE, WorldView};

pub fn user_toggled_point(view: &WorldView) -> Option<Point> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        let Vec2 { x, y } = view.camera.screen_to_world(Vec2 { x, y });

        let x_coord = (x / POINT_SIZE) as i32;
        let y_coord = (y / POINT_SIZE) as i32;
        Some(Point {
            x: x_coord,
            y: y_coord,
        })
    } else {
        None
    }
}

pub fn user_starts_game() -> bool {
    is_key_down(KeyCode::Enter)
}
