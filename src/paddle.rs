use crate::consts::*;
use macroquad::prelude::*;

pub struct Paddle {
    pub rect: Rect,
}

impl Paddle {
    pub fn new() -> Self {
        let size = vec2(50., 10.);
        let pos = vec2(
            screen_width() / 2. - size.x / 2.,
            screen_height() - size.y - OFFSET,
        );
        Self {
            rect: Rect {
                x: pos.x,
                y: pos.y,
                w: size.x,
                h: size.y,
            },
        }
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
