use rand::Rng;

use avian2d::{math::*, prelude::*};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;

mod movement;
mod items;
mod item_list;
mod input;
mod camera;

use movement::prelude::*;
use items::prelude::*;
use input::prelude::*;
use camera::prelude::*;
use item_list::*;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins, 
                PhysicsPlugins::default(),
                CharacterControllerPlugin,
                InputControllerPlugin,
                ItemPlugin,
                CameraControllerPlugin
                //WorldInspectorPlugin::new()
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1200.))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PlayerCamera
    ));
    commands.spawn(
        (
            Name::new("Bunny Boi"),
            Player,
            CharacterControllerBundle::new(Collider::rectangle(10.0, 20.0)).with_movement(
            1400.0,
            0.80,
            400.0,
            (30.0 as Scalar).to_radians(),
            2.8,
            30,
            1
        ),
            //RigidBody::Dynamic, 
            //Collider::rectangle(10.0, 10.0),
            //LockedAxes::ROTATION_LOCKED,
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.75, 0.75),
                    custom_size: Some(Vec2::new(10., 20.)),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    );
    commands.spawn(PlatformBundle::default());
    for _i in 0..10 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-200.0..200.0);
        let y = rng.gen_range(-100.0..600.0);
        let oset = rng.gen_range(-45.0..45.);
        commands.spawn(PlatformBundle::default().with_location(x,y));
        commands.spawn(PlatformBundle::new_wall().with_location(x+oset,y+20.));
    }
    //commands.spawn_batch(
    //    vec![PlatformBundle::default().with_random_location(); 3]
    //);
    commands.spawn(
        ItemBundle::default()
            .with_position(50.,-50.)
            .with_use_accel(|percent| 10. * (percent+0.5).powi(2))
    );
}

#[derive(Bundle, Clone)]
pub struct PlatformBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub sprite_bundle: SpriteBundle
}

impl Default for PlatformBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static, 
            collider: Collider::rectangle(100.0, 10.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(100., 10.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -100., 0.),    
                ..Default::default()
            }
        }
    }
}

impl PlatformBundle {
    pub fn new_wall() -> Self {
        Self {
            rigid_body: RigidBody::Static, 
            collider: Collider::rectangle(10.0, 30.0),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(10., 30.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., -100., 0.),    
                ..Default::default()
            }
        }
    }
    pub fn with_random_location(mut self) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        self.sprite_bundle.transform = Transform::from_xyz(
            x,y,0.
        );
        self
    }
    pub fn with_location(mut self, x: f32, y: f32) -> Self {
        self.sprite_bundle.transform = Transform::from_xyz(
            x,y,0.
        );
        self
    }
}
