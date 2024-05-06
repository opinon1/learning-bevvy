use bevy::prelude::*;

use crate::quadtree::{X_EXTENT, Y_EXTENT};
const LOOK_DIST: f32 = 30f32;

#[derive(Component)]
pub struct Boid {
    pub rotation_speed: f32,
    pub speed: f32,
}

pub struct BoidPlugin;
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_boid);
    }
}

fn update_boid(mut query: Query<(Entity, &Boid, &mut Transform)>) {
    for (_, boid, mut transform) in query.iter_mut() {
        let speed = transform.up() * boid.speed;
        transform.translation -= speed;

        if transform.translation.x.abs() > X_EXTENT {
            transform.translation.x *= -1.0;
        }
        if transform.translation.y.abs() > Y_EXTENT {
            transform.translation.y *= -1.0;
        }
    }
}
