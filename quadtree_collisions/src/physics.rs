use bevy::{a11y::accesskit::Tree, prelude::*};
use std::collections::HashSet;

use crate::quad_trees::{QuadTreeDetect, Quadtree, X_EXTENT, Y_EXTENT};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_physics);
    }
}
fn update_physics(
    mut query: Query<(Entity, &mut Transform, &mut Physics), With<QuadTreeDetect>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

    quadtree: Res<Quadtree>,
) {
    if !keyboard_input.pressed(KeyCode::KeyC) {
        //look for possible collisions
        let mut possible_collisions: HashSet<[Entity; 2]> = HashSet::new();
        for (entity, transform_1, physics_1) in query.iter() {
            //get possible cadidates
            //
            let padding = physics_1.collider_radius * 2.1;

            let mut candidates: Vec<Entity> = Vec::new();

            let area = Rect::new(
                transform_1.translation.x - padding,
                transform_1.translation.y - padding,
                transform_1.translation.x + padding,
                transform_1.translation.y + padding,
            );
            quadtree.query(area, &mut candidates);

            if candidates.len() > 1 {
                if let Some(pos) = candidates.iter().position(|x| *x == entity) {
                    candidates.remove(pos);
                }
                for candidate in candidates.iter() {
                    if candidate.index() < entity.index() {
                        possible_collisions.insert([*candidate, entity]);
                    } else {
                        possible_collisions.insert([entity, *candidate]);
                    }
                }
            }
        }

        for _ in 0..16 {
            //iter possible_collisions
            for possible_collison in possible_collisions.iter() {
                let [(_, mut transform1, mut physics1), (_, mut transform2, mut physics2)] =
                    query.many_mut(*possible_collison);

                let distance = transform1.translation.distance(transform2.translation);
                let depth = (physics1.collider_radius + physics2.collider_radius) - distance;

                if depth >= 0.0 {
                    // Calculate average restitution
                    let restitution = 0.5;

                    // Calculate relative velocity
                    let relative_velocity = physics2.velocity - physics1.velocity;

                    // Calculate velocity component along the normal direction
                    let collision_normal =
                        (transform2.translation - transform1.translation).normalize();
                    let velocity_along_normal = relative_velocity.dot(collision_normal);

                    // Skip if velocities are separating
                    if velocity_along_normal > 0.0 {
                        continue;
                    }

                    // Calculate impulse scalar
                    let impulse_scalar = -(1.0 + restitution) * velocity_along_normal
                        / (1.0 / physics1.mass + 1.0 / physics2.mass);

                    // Apply impulse to the entities' velocities
                    let impulse = collision_normal * impulse_scalar;

                    let v1 = physics1.velocity - impulse / physics1.mass;
                    let v2 = physics2.velocity + impulse / physics2.mass;

                    physics1.velocity = v1;
                    physics2.velocity = v2;
                    //fix their positions
                    let total_inverse_mass = 1.0 / physics1.mass + 1.0 / physics2.mass;

                    let n_pos1 =
                        collision_normal * (depth * (1.0 / physics1.mass) / total_inverse_mass);
                    let n_pos2 =
                        collision_normal * (depth * (1.0 / physics2.mass) / total_inverse_mass);

                    // Correction to push them apart
                    transform1.translation -= n_pos1;
                    transform2.translation += n_pos2;
                    // Apply impulse
                }
            }
        }
    }

    if !keyboard_input.pressed(KeyCode::KeyG) {
        let mut huntsman: Vec<(f32, Vec3)> = Vec::new();
        quadtree.huntsman(&mut huntsman);

        for (_, transform, mut physics) in query.iter_mut() {
            let mut n_acc = Vec3::ZERO;
            for (n, pos) in huntsman.iter() {
                let distance = transform.translation.distance_squared(*pos).min(1.0);
                let normal = (transform.translation - *pos).normalize();

                n_acc += normal * (1.0 / distance) * *n;
            }
            physics.acceleration = n_acc * -0.0002;
        }
    }

    //step dy
    for (_, mut transform, mut physics) in query.iter_mut() {
        let velocity = physics.velocity + physics.acceleration * 0.03;
        physics.velocity = velocity;
        transform.translation += velocity * 0.03;

        if transform.translation.x.abs() > X_EXTENT {
            physics.velocity.x *= 0.7;
            transform.translation.x *= -0.999;
        }
        if transform.translation.y.abs() > Y_EXTENT {
            physics.velocity.y *= 0.7;
            transform.translation.y *= -0.999;
        }
    }
}

#[derive(Component, Debug)]
pub struct Physics {
    pub mass: f32,
    pub collider_radius: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}
