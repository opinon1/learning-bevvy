use bevy::prelude::*;

use crate::quad_trees::{QuadTreeDetect, Quadtree, X_EXTENT, Y_EXTENT};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_physics);
    }
}

fn update_physics(
    mut query: Query<(Entity, &mut Transform, &mut Physics), With<QuadTreeDetect>>,
    time: Res<Time>,
    quadtree: Res<Quadtree>,
) {
    //check for collisions

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

#[derive(Component)]
pub struct Physics {
    pub mass: f32,
    pub collider_radius: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}
