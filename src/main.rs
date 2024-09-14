use std::collections::HashMap;

use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::light::{AmbientLight2d, PointLight2d};

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
//use bevy_inspector_egui::DefaultInspectorConfigPlugin;

macro_rules! import_game_modules {
    ($( $x:ident ),*) => {
        $(
            mod $x {
                pub mod components;
                pub mod systems;
                pub mod prelude {
                    #[allow(unused_imports)]
                    pub use super::components::*;
                    pub use super::systems::*;
                }
            }
            use $x::prelude::*;
        )*
    };
}
//mod camera {
//    pub mod components;
//    pub mod systems;
//    pub mod prelude {
//        pub use super::components::*;
//        pub use super::systems::*;
//    }
//}
//use camera::prelude::*;
import_game_modules!(
    camera,
    graphics,
    health_damage,
    input,
    items,
    level,
    movement,
    player
);

//mod graphics;
//mod health_damage;
//mod input;
//mod items;
//mod level;
//mod movement;
//mod player;

//use graphics::prelude::*;
//use health_damage::prelude::*;
//use input::prelude::*;
//use items::prelude::*;
//use level::prelude::*;
//use movement::prelude::*;
//use player::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EditorPlugin::default(),
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
            InputControllerPlugin,
            ItemPlugin,
            CameraControllerPlugin,
            GraphicsPlugin,
            NPCPlugin,
            LevelPlugin, //WorldInspectorPlugin::new()
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.2, 0.0)))
        .run();
}

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

#[derive(Bundle, Clone, LdtkIntCell)]
pub struct GreenLightingBundle {
    pub light2d: PointLight2d,
}
impl Default for GreenLightingBundle {
    fn default() -> Self {
        Self {
            light2d: PointLight2d {
                radius: 200.0,
                intensity: 1.5,
                cast_shadows: true,
                falloff: 4.5,
                color: Color::linear_rgb(0., 1., 0.2),
            },
        }
    }
}
#[derive(Bundle, Clone, LdtkIntCell)]
pub struct PlatformBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub friction: Friction,
    pub collision_layer: CollisionLayers,
}

impl Default for PlatformBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(16.0, 16.0),
            friction: Friction::new(0.5).with_static_coefficient(0.),
            collision_layer: CollisionLayers::new(GameLayer::GROUND, [GameLayer::CHARACTER]),
        }
    }
}

impl PlatformBundle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(width, height),
            friction: Friction::ZERO,
            collision_layer: CollisionLayers::new(GameLayer::GROUND, [GameLayer::CHARACTER]),
        }
    }
}
