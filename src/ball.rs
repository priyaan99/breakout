use std::f32::consts::PI;

use crate::consts::*;
use macroquad::prelude::*;

pub struct Ball {
    circle: Circle,
    direction: Vec2,
}

impl Ball {
    pub fn new(position: Vec2, direction: Vec2, radius: f32) -> Self {
        Self {
            circle: Circle::new(position.x, position.y, radius),
            direction,
        }
    }

    pub fn init(paddle: Rect) -> Ball {
        Ball::new(
            vec2(paddle.x + paddle.w / 2., paddle.y - BALL_RADIUS - OFFSET),
            Vec2::from_angle(PI / 2.), // 90 degree
            BALL_RADIUS,
        )
    }

    pub fn collided_rect(&self, rect: &Rect) -> bool {
        self.circle.overlaps_rect(rect)
    }

    pub fn clamp_x(&mut self, min: f32, max: f32) {
        self.circle.x = self.circle.x.max(min + self.circle.r);
        self.circle.x = self.circle.x.min(max - self.circle.r);
    }

    pub fn clamp_y(&mut self, min: f32, max: f32) {
        self.circle.y = self.circle.y.max(min + self.circle.r);
        self.circle.y = self.circle.y.min(max - self.circle.r);
    }

    pub fn set_up_angle(&mut self, angle: f32) {
        self.direction = Vec2::from_angle(PI + angle)
    }

    pub fn reverse_y(&mut self) {
        self.direction.y *= -1.;
    }

    pub fn reverse_x(&mut self) {
        self.direction.x *= -1.;
    }

    pub fn update(&mut self) {
        self.circle
            .move_to(self.circle.point() + self.direction * BALL_SPEED * get_frame_time());
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, BALL_COLOR);
    }
}

impl Ball {
    pub fn top(&self) -> f32 {
        self.y() - self.r()
    }

    pub fn bottom(&self) -> f32 {
        self.y() + self.r()
    }

    pub fn left(&self) -> f32 {
        self.x() - self.r()
    }

    pub fn right(&self) -> f32 {
        self.x() + self.r()
    }

    pub fn dir(&self) -> Vec2 {
        self.direction
    }

    pub fn x(&self) -> f32 {
        self.circle.x
    }
    pub fn y(&self) -> f32 {
        self.circle.y
    }
    pub fn r(&self) -> f32 {
        self.circle.r
    }
}
