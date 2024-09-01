#![forbid(unsafe_code)]
#![warn(clippy::all)]

use macroquad::prelude::*;

mod settings;
use crate::settings::Settings;

mod boids;
use crate::boids::Boids;

#[macroquad::main("Boids")]
async fn main() {
    let mut settings = Settings::default();
    let mut birds = Boids::new(settings.population as usize);

    loop {
        clear_background(WHITE);

        birds.update(&settings);
        birds.draw();

        settings.draw_ui();

        next_frame().await;
    }
}
