use crate::consts::*;
use macroquad::prelude::*;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new() -> Self {
        let pos = vec2(
            screen_width() / 2. - PADDLE_SIZE.x / 2.,
            screen_height() - PADDLE_SIZE.y - OFFSET,
        );
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: PADDLE_SIZE.x,
                h: PADDLE_SIZE.y,
            },
        }
    }
    pub fn x(&self) -> f32 {
        self.rect.x
    }
    pub fn y(&self) -> f32 {
        self.rect.y
    }

    pub fn w(&self) -> f32 {
        self.rect.w
    }
    pub fn h(&self) -> f32 {
        self.rect.h
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn move_left(&mut self) {
        self.rect.x -= PADDLE_SPEED * get_frame_time();
    }

    pub fn move_right(&mut self) {
        self.rect.x += PADDLE_SPEED * get_frame_time();
    }

    fn clamp_x(&mut self) {
        self.rect.x = self.rect.x.clamp(0., screen_width() - self.rect.w);
    }

    pub fn update(&mut self) {
        self.clamp_x();
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            PADDLE_COLOR,
        );
    }
}
