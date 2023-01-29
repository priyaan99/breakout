pub mod ball;
pub mod brick;
pub mod paddle;

pub mod consts {
    use macroquad::color::*;

    pub const OFFSET: f32 = 10.;
    pub const BLOCK_SIZE: f32 = 30.;
    pub const ROWS: f32 = 5.;
    pub const COLS: f32 = 10.;
    pub const WIDTH: f32 = BLOCK_SIZE * COLS;
    pub const HEIGHT: f32 = BLOCK_SIZE * ROWS * 2.5;

    pub const PADDLE_SPEED: f32 = 200.;
    pub const PADDLE_COLOR: Color = BLUE;

    pub const BALL_RADIUS: f32 = 10.;
    pub const BALL_SPEED: f32 = 100.;
    pub const BALL_COLOR: Color = SKYBLUE;
}
