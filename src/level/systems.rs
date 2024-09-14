use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    GraphicsBundle, GreenLightingBundle, HahaBundle, ItemBundle, PlatformBundle, Player,
    PlayerBundle,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            //.register_ldtk_entity::<PlayerBundle>("Player")
            //.register_ldtk_entity::<HahaBundle>("Haha")
            //.register_default_ldtk_entity_for_layer::<ItemBundle>("Items")
            .register_ldtk_int_cell_for_layer::<PlatformBundle>("TileIntGrid", 1)
            //.register_ldtk_int_cell_for_layer::<GreenLightingBundle>("IntGridLighting", 1)
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_systems(Startup, spawn_level);
    }
}

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/level0.ldtk"),
        ..Default::default()
    });
}
