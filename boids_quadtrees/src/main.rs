mod boid;
mod quadtree;

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::{thread_rng, Rng};

use boid::{Boid, BoidPlugin};
use quadtree::{QuadTreeDetect, QuadtreePlugin, X_EXTENT, Y_EXTENT};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(QuadtreePlugin)
        .add_plugins(BoidPlugin)
        .add_systems(Startup, (spawn_particles, spawn_camera))
        .add_systems(Update, print_fps)
        .run()
}

const N_ENTITIES: usize = 10_000;

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rgen = thread_rng();

    let mesh_hande = Mesh2dHandle(meshes.add(Rectangle::new(2.0, 5.0)));
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
                )
                .with_rotation(Quat::from_rotation_z(rgen.gen_range(0.0..(2.0 * PI)))),

                ..default()
            },
            QuadTreeDetect,
            Boid {
                rotation_speed: 1f32,
                speed: 30f32,
            },
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn print_fps(time: Res<Time>) {
    println!("{}", 1.0 / time.delta_seconds())
}
