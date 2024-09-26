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
    farming,
    health_damage,
    input,
    interactables,
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
            InteractablePlugin,
            ItemPlugin,
            CameraControllerPlugin,
            GraphicsPlugin,
            FarmingPlugin,
            HealthPlugin,
            EnemyPlugin,
            PlayerPlugin,
            LevelPlugin, //WorldInspectorPlugin::new()
        ))
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.2, 0.0)))
        .run();
}
