use breakout::sound::Sounds;
use macroquad::audio::play_sound_once;
use macroquad::prelude::*;

use breakout::consts::*;
use breakout::play::{Outcome, Play};

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
    println!("width {} \nheight {}", WIDTH, HEIGHT);
    let mut out = true;
    let sounds = Sounds::new().await;
    let mut play = Play::init(sounds);
    let mut result: Option<Outcome> = None;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if out {
            if is_key_pressed(KeyCode::Enter) {
                out = false;
                play = Play::init(sounds);
                play_sound_once(sounds.click);
            }
        } else {
            play.update();

            result = play.finish();

            if let Some(_) = result {
                out = true;
            }
        }

        clear_background(WHITE);
        if out {
            let press_enter_str = "Press Enter to Play.";
            const FONT_SIZE: u16 = 20;
            let press_enter_measure = measure_text(press_enter_str, None, FONT_SIZE, 1.);

            if let Some(r) = result {
                match r {
                    Outcome::Win => {
                        draw_text("You Win.", WIDTH / 2. - 50., HEIGHT / 1. / 3., 30., RED);
                        draw_text(
                            press_enter_str,
                            WIDTH / 2. - press_enter_measure.width / 2.,
                            HEIGHT / 2.,
                            FONT_SIZE as f32,
                            RED,
                        )
                    }
                    Outcome::Loose => {
                        draw_text("You Loose.", WIDTH / 2. - 50., HEIGHT / 1. / 3., 30., RED);
                        draw_text(
                            press_enter_str,
                            WIDTH / 2. - press_enter_measure.width / 2.,
                            HEIGHT / 2.,
                            FONT_SIZE as f32,
                            RED,
                        )
                    }
                }
            } else {
                draw_text(
                    press_enter_str,
                    WIDTH / 2. - press_enter_measure.width / 2.,
                    HEIGHT / 2.,
                    FONT_SIZE as f32,
                    RED,
                )
            }
        } else {
            play.draw();
        }

        next_frame().await
    }
}
