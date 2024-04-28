use bevy::prelude::*;

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub model: SceneBundle,
    pub acceleration: Acceleration,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}
impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
