mod asset_loader;
mod asteroids;
mod camera;
mod collider_detection;
mod debug;
mod despawn;
mod lighting;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collider_detection::CollitionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use lighting::LightingPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(LightingPlugin)
        //.add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CollitionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
