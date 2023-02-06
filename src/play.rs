use crate::{ball::*, brick::Brick, consts::*, paddle::*, sound::Sounds};
use macroquad::{audio::play_sound_once, prelude::*};

#[derive(Clone, Copy)]
pub enum Outcome {
    Win,
    Loose,
}

pub struct Play {
    result: Option<Outcome>,
    life: i32,
    score: usize,
    paddle: Paddle,
    ball: Ball,
    briks: Vec<Brick>,
    sounds: Sounds,
}

impl Play {
    pub fn init(sounds: Sounds) -> Self {
        let paddle = Paddle::new();
        let ball = Ball::init(paddle.rect);

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

        Self {
            sounds,
            result: None,
            life: 3,
            score: 0,
            paddle,
            ball,
            briks,
        }
    }

    pub fn update(&mut self) {
        self.paddle_movement();

        // collisions
        self.ball_vs_bricks();
        self.ball_vs_paddle();
        self.ball_vs_wall();
        // ball out of field

        if self.briks.iter().filter(|b| b.active()).count() == 0 {
            self.result = Some(Outcome::Win)
        }

        if self.ball.y() > HEIGHT {
            if self.life == 1 {
                self.result = Some(Outcome::Loose)
            } else {
                self.life -= 1;
                self.ball = Ball::init(self.paddle.rect());
            }
        }

        self.paddle.update();
        self.ball.update(self.paddle.rect());
    }

    fn paddle_movement(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.paddle.move_left();
        } else if is_key_down(KeyCode::Right) {
            self.paddle.move_right();
        }
    }

    fn ball_vs_wall(&mut self) {
        if self.ball.left() < 0. || self.ball.right() > WIDTH {
            self.ball.reverse_x();
            self.ball.clamp_x(0., WIDTH);
            play_sound_once(self.sounds.ball_vs_paddle);
        }

        if self.ball.top() < 0. {
            self.ball.reverse_y();
            self.ball.clamp_y(0., HEIGHT + 100.);
            play_sound_once(self.sounds.ball_vs_paddle);
        }
    }

    fn ball_vs_paddle(&mut self) {
        if self.ball.collided_rect(&self.paddle.rect())
            && self.ball.y() <= self.paddle.y() + 2.
            && self.ball.dir().y > 0.
        {
            let where_collided = self.ball.x() - (self.paddle.x() - self.ball.r());
            let sum_length = self.paddle.w() + self.ball.r() * 2.;

            let angle = (where_collided / sum_length * 180.)
                .clamp(30., 160.)
                .to_radians();

            self.ball.set_up_angle(angle);
            play_sound_once(self.sounds.ball_vs_paddle);
        }
    }

    fn ball_vs_bricks(&mut self) {
        for b in self.briks.iter_mut() {
            if b.active() && self.ball.collided_rect(&b.rect()) {
                b.deacitvate();
                let thresshold = 5.;
                // hit below
                if self.ball.top() >= b.bottom() - thresshold && self.ball.dir().y < 0. {
                    self.ball.reverse_y()
                } else if self.ball.bottom() <= b.top() + thresshold && self.ball.dir().y > 0. {
                    // hit above
                    self.ball.reverse_y();
                } else if self.ball.dir().x > 0. {
                    // hit left
                    self.ball.reverse_x();
                } else if self.ball.dir().x < 0. {
                    // hit right
                    self.ball.reverse_x();
                }

                self.score += 1;

                play_sound_once(self.sounds.ball_vs_briks);
            }
        }
    }

    pub fn draw(&self) {
        self.paddle.draw();
        self.ball.draw();

        for b in self.briks.iter() {
            if b.active() {
                b.draw();
            }
        }

        // draw remaining balls
        for i in 1..=self.life {
            draw_rectangle(
                (OFFSET + BALL_RADIUS) * i as f32,
                HEIGHT * 6. / 7.,
                BALL_RADIUS,
                BALL_RADIUS,
                Color { a: 0.8, ..SKYBLUE },
            )
        }
    }

    pub fn finish(&self) -> Option<Outcome> {
        self.result
    }
}
