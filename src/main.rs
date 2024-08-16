use rand::Rng;

use avian2d::{math::*, prelude::*};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;

mod movement;
mod items;
mod item_list;
mod input;
mod camera;
mod graphics;

use movement::prelude::*;
use items::prelude::*;
use input::prelude::*;
use camera::prelude::*;
use graphics::prelude::*;
use item_list::*;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()), 
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    //let hehe_texture_handle: Handle<Image> = asset_server.load("sprites/hehe.png");
    //let hehe_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(10, 20), 6, 2, None, None);
    //let hehe_texture_atlas_layout = texture_atlas_layouts.add(hehe_atlas_layout);

    commands.spawn((
        Camera2dBundle::default(),
        PlayerCamera
    ));
    commands.spawn(
        (
            Name::new("Hehe"),
            Player,
            CharacterControllerBundle::new(Collider::rectangle(10.0, 20.0)).with_movement(
            1400.0,
            0.80,
            400.0,
            (30.0 as Scalar).to_radians(),
            2.8,
            30,
            2
        ),
            //RigidBody::Dynamic, 
            //Collider::rectangle(10.0, 10.0),
            //LockedAxes::ROTATION_LOCKED,
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GraphicsBundle::new(
                asset_server.load("sprites/hehe.png"),
                texture_atlas_layouts,
                UVec2::new(10, 20),
                6,
                2,
                Vec2::new(0., -10.)
            )
            //SpriteBundle {
            //    texture: hehe_texture_handle,
            //    transform: Transform::from_xyz(0., -10., 0.).with_scale(Vec3::splat(2.)),
            //    ..default()
            //},
            //TextureAtlas {
            //    layout: hehe_texture_atlas_layout,
            //    index: 1
            //}
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
            .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
            .with_use_time(250)
            .with_swing_desc(4. * PI / 3., PI / 6., 4.* PI / 3.)
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
                    ..default()
                },
                transform: Transform::from_xyz(0., -100., 0.),    
                ..default()
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
