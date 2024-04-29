use bevy::prelude::*;

const MAX_DISTANCE: f32 = 100f32;

pub struct DespawnPlugin;
impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation().distance(Vec3::ZERO) > MAX_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
