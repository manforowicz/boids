//! Manages the boids

use crate::settings::Settings;
use kiddo::{KdTree, SquaredEuclidean};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
}

impl Boid {
    fn new() -> Self {
        Self {
            pos: Vec2::new(
                rand::gen_range(0., screen_width()),
                rand::gen_range(0., screen_height()),
            ),
            vel: Vec2::new(rand::gen_range(-50., 50.), rand::gen_range(-50., 50.)),
            color: DARKGRAY,
        }
    }

    fn update_prey(
        &self,
        birds: &[Boid],
        predators: &[Boid],
        tree: &KdTree<f32, 2>,
        settings: &Settings,
        delta: f32,
    ) -> Self {
        let mut neighbor_force = Vec2::ZERO;
        let neighbors = tree.nearest_n::<SquaredEuclidean>(&self.pos.into(), 6);
        for other in &neighbors {
            let neighbor = &birds[other.item as usize];
            let dist = other.distance.sqrt();

            // separation
            neighbor_force += (dist - settings.spacing_goal).min(0.)
                * settings.separation_weight
                * 8.0
                * (neighbor.pos - self.pos).normalize_or_zero();

            // alignment
            neighbor_force += (neighbor.vel - self.vel) * settings.alignment_weight * 0.15;

            // cohesion
            neighbor_force += (neighbor.pos - self.pos) * settings.cohesion_weight * 0.4;
        }

        if !neighbors.is_empty() {
            neighbor_force /= neighbors.len() as f32;
        }

        let mut predator_force = Vec2::ZERO;

        for predator in predators {
            let dist = self.pos.distance(predator.pos);
            // dis-alignment
            let away_vector = if predator.vel.perp().dot(predator.pos - self.pos) > 0. {
                predator.vel.perp()
            } else {
                -predator.vel.perp()
            };

            predator_force += (dist - 120.).min(0.) * away_vector * 0.05;
        }

        if !predators.is_empty() {
            predator_force /= predators.len() as f32;
        }

        let other_force = self.calc_common_forces(settings);

        // apply the forces
        let total_force = neighbor_force + predator_force + other_force;

        // cool color mapping
        let color = Color::new(
            0.008 * total_force.length(),
            0.01 * self.vel.length(),
            0.,
            1.,
        );

        let vel = self.vel + total_force * delta;

        Self {
            pos: self.pos + vel * delta,
            vel,
            color,
        }
    }

    fn update_predator(
        &self,
        birds: &[Boid],
        _predators: &[Boid],
        tree: &KdTree<f32, 2>,
        settings: &Settings,
        delta: f32,
    ) -> Self {
        let mut neighbor_force = Vec2::ZERO;
        let neighbors = tree.nearest_n::<SquaredEuclidean>(&self.pos.into(), 6);
        for other in &neighbors {
            let neighbor = &birds[other.item as usize];

            // cohesion
            neighbor_force += (neighbor.pos - self.pos) * settings.cohesion_weight * 0.02;
        }

        let other_force = self.calc_common_forces(settings);

        let vel = self.vel + (neighbor_force + other_force) * delta;

        Self {
            pos: self.pos + vel * delta,
            vel,
            color: BLACK,
        }
    }

    fn calc_common_forces(&self, settings: &Settings) -> Vec2 {
        // speed target
        let speed_force = self.vel.normalize_or_zero()
            * (settings.target_speed * 15. - self.vel.length())
            * settings.speed_weight
            * 0.4;

        // wall force
        let wall_force_x = (50. - self.pos.x).max(0.) + (screen_width() - 50. - self.pos.x).min(0.);
        let wall_force_y =
            (50. - self.pos.y).max(0.) + (screen_height() - 50. - self.pos.y).min(0.);
        let wall_force = 5. * Vec2::new(wall_force_x, wall_force_y);

        speed_force + wall_force
    }

    fn draw(&self, size: f32) {
        let rot = self.vel.to_angle();
        draw_triangle(
            polar_to_cartesian(size, rot) + self.pos,
            polar_to_cartesian(size, rot - 2.4) + self.pos,
            polar_to_cartesian(size, rot + 2.4) + self.pos,
            self.color,
        );
    }
}

pub struct Boids {
    birds: Vec<Boid>,

    /// Stores indexes to `birds`.
    tree: KdTree<f32, 2>,

    predators: Vec<Boid>,
}

impl Boids {
    pub fn new(quantity: usize) -> Self {
        Self {
            birds: (0..quantity).map(|_| Boid::new()).collect(),
            tree: KdTree::with_capacity(quantity),
            predators: (0..1).map(|_| Boid::new()).collect(),
        }
    }

    pub fn update(&mut self, settings: &Settings) {
        let delta = get_frame_time().clamp(0., 0.1);

        if self.birds.len() != settings.population as usize {
            self.birds
                .resize_with(settings.population as usize, Boid::new)
        }

        if self.predators.is_empty() == settings.predator {
            let num_predators = if settings.predator { 1 } else { 0 };
            self.predators.resize_with(num_predators, Boid::new)
        }

        if settings.paused {
            return;
        }

        self.tree = KdTree::from_iter(
            self.birds
                .iter()
                .enumerate()
                .map(|(i, bird)| (bird.pos.into(), i as u64)),
        );

        for i in 0..self.birds.len() {
            self.birds[i] = self.birds[i].update_prey(
                &self.birds,
                &self.predators,
                &self.tree,
                settings,
                delta,
            );
        }

        for i in 0..self.predators.len() {
            self.predators[i] = self.predators[i].update_predator(
                &self.birds,
                &self.predators,
                &self.tree,
                settings,
                delta,
            );
        }
    }

    pub fn draw(&self) {
        for bird in &self.birds {
            bird.draw(8.);
        }

        for bird in &self.predators {
            bird.draw(12.);
        }
    }
}
