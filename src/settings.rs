//! Manages settings and settings UI.

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use std::ops::Range;

pub struct Settings {
    pub paused: bool,
    pub predator: bool,
    pub population: f32,
    pub spacing_goal: f32,
    pub separation_weight: f32,
    pub cohesion_weight: f32,
    pub alignment_weight: f32,
    pub target_speed: f32,
    pub speed_weight: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            paused: true,
            predator: true,
            population: screen_width() * screen_height() * 0.0006,
            spacing_goal: 40.,
            separation_weight: 5.,
            cohesion_weight: 5.,
            alignment_weight: 5.,
            target_speed: 5.,
            speed_weight: 5.,
        }
    }
}

macro_rules! slider {
    ($ui:expr, $settings:ident. $name:ident, $start:expr, $stop:expr) => {
        $ui.slider(
            hash!(),
            &stringify!($name).replace("_", " "),
            Range {
                start: $start,
                end: $stop,
            },
            &mut $settings.$name,
        );
    };
}

impl Settings {
    pub fn draw_ui(&mut self) {
        widgets::Popup::new(hash!(), vec2(250., 180.)).ui(&mut root_ui(), |ui| {
            ui.checkbox(hash!(), "START!", &mut self.paused);
            ui.checkbox(hash!(), "Predator", &mut self.predator);
            slider!(ui, self.population, 0., 2000.);
            slider!(ui, self.spacing_goal, 0., 100.);
            slider!(ui, self.separation_weight, 0., 10.);
            slider!(ui, self.cohesion_weight, 0., 10.);
            slider!(ui, self.alignment_weight, 0., 10.);
            slider!(ui, self.target_speed, 0., 10.);
            slider!(ui, self.speed_weight, 0., 10.);

            self.population = self.population.round();
        });

        draw_rectangle(0., 0., 300., 180., Color::new(0.95, 0.95, 0.95, 0.8));
    }
}
