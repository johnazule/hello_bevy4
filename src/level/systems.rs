use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{AnimationList, GraphicsBundle, GraphicsState, GreenLightingBundle, HahaBundle, PlatformBundle, Player, PlayerBundle, PlayerGraphicsBundle, StateAnimation};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<HahaBundle>("Haha")
            .register_ldtk_int_cell_for_layer::<PlatformBundle>("TileIntGrid", 1)
            .register_ldtk_int_cell_for_layer::<GreenLightingBundle>("IntGridLighting", 1)
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_systems(Startup, spawn_level);
            //.add_systems(Update, process_entity);
    }
}

pub fn spawn_level(
   mut commands: Commands,
   asset_server: Res<AssetServer>
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/level0.ldtk"),
        ..Default::default()
    });
}

//fn process_entity(
//    mut commands: Commands,
//    new_entity_instances: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
//    asset_server: Res<AssetServer>,
//    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
//)
//{
//    for (entity, entity_instance, transform) in new_entity_instances.iter() {
//        info!("Here");
//        match entity_instance.identifier.as_str() {
//            "Player" => {
//
//            },
//            _ => {
//
//            }
//        };
//        info!("Here!!!");
//        commands
//            .entity(entity)
//            .insert(PlayerBundle::default())
//            .insert(PlayerGraphicsBundle {
//                graphics_bundle: GraphicsBundle::new(
//                                     asset_server.load("sprites/hehe.png"),
//                                     &mut texture_atlas_layouts,
//                                     UVec2::new(10, 20),
//                                     6,
//                                     3,
//                                     Vec2::new(0., -10.)
//                                 ).with_transform(*transform), 
//                state: GraphicsState::Falling,
//                animation_list: AnimationList(HashMap::from([
//                        (GraphicsState::Idle, StateAnimation::new_timer(12, 17, 450)),
//                        (GraphicsState::Running, StateAnimation::new_timer(0, 5, 300)),
//                        (GraphicsState::Jumping, StateAnimation::new_velocity_list(6, 11, vec![
//                                                                                   100.,
//                                                                                   200.,
//                                                                                   300.,
//                                                                                   400.,
//                                                                                   500.,
//                        ])),
//                        (GraphicsState::Falling, StateAnimation::new_velocity_list(11, 6, vec![
//                                                                                   -10.,
//                                                                                   -100.,
//                                                                                   -220.,
//                                                                                   -230.,
//                                                                                   -250.,
//                        ])),
//                ])),
//            });
//            //.insert(SpriteBundle {
//            //    texture: assets.load("sprites/hehe.png"),
//            //    transform: *transform,
//            //    ..default()
//            //});
//    }
//}
