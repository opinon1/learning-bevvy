mod physics;
mod quad_trees;

use crate::physics::{Physics, PhysicsPlugin};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use quad_trees::{QuadTreeDetect, QuadtreePlugin, X_EXTENT, Y_EXTENT};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(QuadtreePlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, (spawn_particles, spawn_camera))
        .add_systems(Update, print_fps)
        .run()
}

const MAX_SPEED: f32 = 100f32;
const RADIUS: f32 = 2f32;
const N_ENTITIES: usize = 10_000;

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rgen = thread_rng();

    let mesh_hande = Mesh2dHandle(meshes.add(Circle::new(RADIUS)));
    let color = Color::WHITE;

    for _ in 0..N_ENTITIES {
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
                acceleration: Vec3::new(0.0, 0.0, 0.0),
            },
            QuadTreeDetect,
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn print_fps(time: Res<Time>) {
    println!("{}", 1.0 / time.delta_seconds())
}
