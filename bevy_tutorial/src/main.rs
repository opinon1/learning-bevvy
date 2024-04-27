use bevy::prelude::*;

#[derive(Component)]
struct Velocity {
    pub value: Vec3,
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run()
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::new(0.0, 0.0, 0.0),
        },
    ));
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.value;
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        println!("{:?} at position: {:?}", entity, transform.translation)
    }
}
