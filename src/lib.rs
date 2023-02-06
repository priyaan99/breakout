pub mod ball;
pub mod brick;
pub mod paddle;
pub mod play;

pub mod consts {
    use macroquad::color::*;
    use macroquad::prelude::{vec2, Vec2};

    pub const OFFSET: f32 = 10.;
    pub const BLOCK_SIZE: f32 = 50.;
    pub const ROWS: f32 = 5.;
    pub const COLS: f32 = 10.;
    pub const WIDTH: f32 = BLOCK_SIZE * COLS;
    pub const HEIGHT: f32 = BLOCK_SIZE * ROWS * 2.2;

    pub const PADDLE_SPEED: f32 = 250.;
    pub const PADDLE_SIZE: Vec2 = vec2(WIDTH * 1. / 6., OFFSET);
    pub const PADDLE_COLOR: Color = BLUE;

    pub const BALL_RADIUS: f32 = 10.;
    pub const BALL_SPEED: f32 = 300.;
    pub const BALL_COLOR: Color = SKYBLUE;
}
