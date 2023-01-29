use crate::consts::*;
use macroquad::prelude::*;

pub struct Ball {
    position: Vec2,
    direction: Vec2,
    radius: f32,
}

impl Ball {
    pub fn new(position: Vec2, direction: Vec2, radius: f32) -> Self {
        Self {
            position,
            direction,
            radius,
        }
    }

    pub fn update(&mut self) {
        self.position += self.direction * BALL_SPEED * get_frame_time();
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, BALL_COLOR);
    }
}
