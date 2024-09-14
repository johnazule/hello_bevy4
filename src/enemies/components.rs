use std::collections::HashMap;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    AnimationList, CharacterControllerBundle, Facing, FallBundle, GraphicsState, HealthBundle,
    JumpBundle, PlayerGraphicsBundle, RunBundle, StateAnimation,
};

#[derive(Bundle, LdtkEntity)]
pub struct HahaBundle {
    name: Name,
    controller: CharacterControllerBundle,
    health: HealthBundle,
    player_graphics_bundle: PlayerGraphicsBundle,
    #[sprite_sheet_bundle("sprites/haha.png", 20, 50, 4, 2, 1, 0, 0)]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

impl Default for HahaBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Haha"),
            controller: CharacterControllerBundle::new(Collider::rectangle(20., 50.))
                .with_movement(
                    0.8,
                    0.8,
                    RunBundle::default(),
                    JumpBundle::default(),
                    FallBundle::default(),
                    40.,
                    30,
                ),
            health: HealthBundle::new(100., [[0., 0.], [1., 1.]], 300),
            player_graphics_bundle: PlayerGraphicsBundle {
                facing: Facing::default(),
                state: GraphicsState::Falling,
                animation_list: AnimationList(HashMap::from([
                    (GraphicsState::Idle, StateAnimation::new_timer(0, 3, 450)),
                    (GraphicsState::Running, StateAnimation::new_timer(4, 7, 300)),
                    //(GraphicsState::Jumping, StateAnimation::new_velocity_list(6, 11, vec![
                    //                                                           100.,
                    //                                                           200.,
                    //                                                           300.,
                    //                                                           400.,
                    //                                                           500.,
                    //])),
                    //(GraphicsState::Falling, StateAnimation::new_velocity_list(11, 6, vec![
                    //                                                           -10.,
                    //                                                           -100.,
                    //                                                           -220.,
                    //                                                           -230.,
                    //                                                           -250.,
                    //])),
                ])),
            },
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
        }
    }
}
