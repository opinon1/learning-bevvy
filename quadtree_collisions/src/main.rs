mod quad_trees;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use quad_trees::Quadtree;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::{X_EXTENT, Y_EXTENT};
use quad_trees::{QuadTreeDetect, QuadtreePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(QuadtreePlugin)
        .add_systems(Startup, (spawn_particles, spawn_camera))
        .add_systems(Update, update_physics)
        .run()
}

const RADIUS: f32 = 1.0f32;

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rgen = thread_rng();

    let mesh_hande = Mesh2dHandle(meshes.add(Circle::new(RADIUS)));
    let color = Color::WHITE;

    for _ in 0..10_000 {
        // Distribute colors evenly across the rainbow.
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh_hande.clone(),
                material: materials.add(color),
                transform: Transform::from_xyz(
                    rgen.gen_range(-X_EXTENT..X_EXTENT),
                    rgen.gen_range(-Y_EXTENT..Y_EXTENT),
                    0.0,
                ),
                ..default()
            },
            Physics {
                mass: RADIUS * RADIUS * PI,
                collider_radius: RADIUS,
                velocity: Vec3::new(
                    rgen.gen_range(-MAX_SPEED..MAX_SPEED),
                    rgen.gen_range(-MAX_SPEED..MAX_SPEED),
                    0.0,
                ),
                acceleration: Vec3::ZERO,
            },
            QuadTreeDetect,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
