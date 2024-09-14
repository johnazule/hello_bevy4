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
                    #[allow(unused_imports)]
                    pub use super::systems::*;
                }
            }
            use $x::prelude::*;
        )*
    };
}
import_game_modules!(
    camera,
    graphics,
    enemies,
    health_damage,
    input,
    items,
    level,
    movement,
    player
);

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
            HealthPlugin,
            EnemyPlugin,
            PlayerPlugin,
            LevelPlugin, //WorldInspectorPlugin::new()
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.2, 0.0)))
        .run();
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
