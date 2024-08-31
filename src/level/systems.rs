use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin);
    }
}
