use bevy::prelude::*;

use crate::quadtree::{Quadtree, X_EXTENT, Y_EXTENT};
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

fn update_boid(
    mut query: Query<(Entity, &Boid, &mut Transform)>,
    quadtree: Res<Quadtree>,
    time: Res<Time>,
) {
    //update angle:
    for (_, boid, mut transform) in query.iter_mut() {
        let padding = 10.0;

        let mut transforms: Vec<Transform> = Vec::new();

        let area = Rect::new(
            transform.translation.x - padding,
            transform.translation.y - padding,
            transform.translation.x + padding,
            transform.translation.y + padding,
        );
        quadtree.query(area, &mut transforms);

        let length = (transforms.len() - 1) as f32;
        let mut average = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
        for transform_2 in transforms.iter() {
            if *transform != *transform_2 {
                average.z += transform_2.rotation.z;
                average.w += transform_2.rotation.w;
            }
        }
        transform.rotation.z += average.z / length * time.delta_seconds();
        transform.rotation.w += average.w / length * time.delta_seconds();
    }

    //update pos
    for (_, boid, mut transform) in query.iter_mut() {
        let speed = transform.up() * boid.speed;
        transform.translation -= speed * time.delta_seconds();

        if transform.translation.x.abs() > X_EXTENT {
            transform.translation.x *= -1.0;
        }
        if transform.translation.y.abs() > Y_EXTENT {
            transform.translation.y *= -1.0;
        }
    }
}
