use super::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<HahaBundle>("Haha");
    }
}
