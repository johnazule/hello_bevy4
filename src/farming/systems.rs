use bevy::prelude::*;
use bevy_ecs_ldtk::app::LdtkEntityAppExt;

use crate::FarmBundle;

pub struct FarmingPlugin;

impl Plugin for FarmingPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<FarmBundle>("Farm");
    }
}

//fn interactable_visual_feedback(mut query: Query) {}
//fn plant(mut query: Query) {}
