use bevy::prelude::*;

use crate::quad_trees::{QuadTreeDetect, Quadtree};
use crate::{X_EXTENT, Y_EXTENT};

fn update_physics(
    mut query: Query<(Entity, &mut Transform, &mut Physics), With<QuadTreeDetect>>,
    time: Res<Time>,
    quadtree: Res<Quadtree>,
) {
    //check for collisions
    for (entity, mut transform, mut physics) in query.iter_mut() {
        for (entity_2, mut transform_2, mut physics) in
            quadtree.query(transform.translation).iter_mut()
        {}
    }

    //step dy
    for (_, mut transform, mut physics) in query.iter_mut() {
        let velocity = physics.velocity + physics.acceleration * time.delta_seconds();
        physics.velocity = velocity;
        transform.translation += velocity * time.delta_seconds();

        if transform.translation.x.abs() > X_EXTENT {
            physics.velocity.x *= -1.0;
            transform.translation.x *= 0.99;
        }
        if transform.translation.y.abs() > Y_EXTENT {
            physics.velocity.y *= -1.0;
            transform.translation.y *= 0.99;
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Physics {
    mass: f32,
    collider_radius: f32,
    velocity: Vec3,
    acceleration: Vec3,
}
