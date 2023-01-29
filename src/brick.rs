use macroquad::{color::Color, prelude::Rect, shapes::draw_rectangle};

pub struct Brick {
    rect: Rect,
    color: Color,
}

impl Brick {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self { rect, color }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        )
    }
}
