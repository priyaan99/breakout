use macroquad::audio::{load_sound, Sound};

#[derive(Clone, Copy)]
pub struct Sounds {
    pub ball_vs_briks: Sound,
    pub ball_vs_paddle: Sound,
    pub click: Sound,
}

impl Sounds {
    pub async fn new() -> Self {
        let back = load_sound("assets/back_002.ogg").await.unwrap();
        let error = load_sound("assets/error_007.ogg").await.unwrap();
        let force_field = load_sound("assets/forceField_003.ogg").await.unwrap();
        Self {
            ball_vs_briks: error,
            ball_vs_paddle: force_field,
            click: back,
        }
    }
}
