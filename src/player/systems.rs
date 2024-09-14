use super::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Update, process_player);
    }
}

fn process_player(
    //mut commands: Commands,
    mut new_entity_instances: Query<(Entity, &mut Transform), Added<Player>>,
) {
    for (entity, mut transform) in new_entity_instances.iter_mut() {
        transform.translation.z = 100.;
    }
}
