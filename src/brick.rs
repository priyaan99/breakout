use macroquad::{color::Color, prelude::Rect, shapes::draw_rectangle};

pub struct Brick {
    rect: Rect,
    color: Color,
    active: bool,
}

impl Brick {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color,
            active: true,
        }
    }

    pub fn top(&self) -> f32 {
        self.rect.y
    }

    pub fn bottom(&self) -> f32 {
        self.rect.y + self.rect.h
    }

    pub fn left(&self) -> f32 {
        self.rect.x
    }

    pub fn right(&self) -> f32 {
        self.rect.x + self.rect.w
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn deacitvate(&mut self) {
        self.active = false
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
