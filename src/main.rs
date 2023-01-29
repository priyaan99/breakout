use macroquad::prelude::*;

use breakout::consts::*;
use breakout::{ball::*, brick::Brick, paddle::*};

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout".to_owned(),
        fullscreen: false,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut paddle = Paddle::new();
    let mut ball = Ball::new(
        vec2(
            paddle.rect().x + paddle.rect().w / 2.,
            paddle.rect().y - BALL_RADIUS,
        ),
        Vec2::NEG_Y,
        BALL_RADIUS,
    );

    let mut briks = vec![];
    for row in 0..ROWS as usize {
        for col in 0..COLS as usize {
            let brick_rect = Rect::new(
                col as f32 * BLOCK_SIZE,
                row as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
            );
            let brick_color = if (row + col) % 2 == 0 {
                GRAY
            } else {
                LIGHTGRAY
            };
            briks.push(Brick::new(brick_rect, brick_color));
        }
    }

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Left) {
            paddle.move_left();
        } else if is_key_down(KeyCode::Right) {
            paddle.move_right();
        }

        paddle.update();
        ball.update();

        clear_background(WHITE);
        paddle.draw();
        ball.draw();

        for b in briks.iter() {
            b.draw();
        }

        next_frame().await
    }
}
