use std::{collections::HashMap, time::Duration};

use bevy_ecs_ldtk::prelude::*;
use bevy_light_2d::light::{AmbientLight2d, PointLight2d, PointLight2dBundle};
use rand::Rng;

use avian2d::{math::*, prelude::*};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{prelude::*, sprite::Anchor};

mod camera;
mod graphics;
mod input;
mod items;
mod level;
mod movement;
mod npcs;

mod item_list;

use camera::prelude::*;
use graphics::prelude::*;
use input::prelude::*;
use items::prelude::*;
use level::prelude::*;
use movement::prelude::*;
use npcs::prelude::*;

use item_list::*;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.set(ImagePlugin::default_nearest()), 
                PhysicsPlugins::default(),
                CharacterControllerPlugin,
                InputControllerPlugin,
                ItemPlugin,
                CameraControllerPlugin,
                GraphicsPlugin,
                NPCPlugin,
                LevelPlugin
                //WorldInspectorPlugin::new()
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1200.))
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.2, 0.0)))
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
        AmbientLight2d {
            brightness: 0.2,
            color: Color::linear_rgb(1.00, 0.8, 0.5),
        },
        PlayerCamera
    ));
    //commands.spawn(
    //    (
    //        Name::new("Hehe"),
    //        Player,
    //        CharacterControllerBundle::new(Collider::rectangle(10.0, 20.0)).with_movement(
    //            0.80,
    //            0.50,
    //            RunBundle::new(
    //                20.,
    //                380.,
    //                550,
    //                Vec2::new(0.5, 0.5),
    //                Vec2::new(0.5, 0.5)
    //            ),
    //            JumpBundle::new(
    //                800.,
    //                200,
    //                3,
    //                Vec2::new(0., 1.),
    //                Vec2::new(0.7, 0.9),
    //            ),
    //            FallBundle::new(
    //                // TODO: Get rid of initial fall speed??
    //                -0.,
    //                //-800.,
    //                -0.,
    //                550,
    //                Vec2::new(0.0, 1.0),
    //                Vec2::new(0.50, 1.0)
    //            ),
    //            (30.0 as Scalar).to_radians(),
    //            100,
    //        ),
    //        //RigidBody::Dynamic, 
    //        //Collider::rectangle(10.0, 10.0),
    //        //LockedAxes::ROTATION_LOCKED,
    //        Friction::ZERO.with_combine_rule(CoefficientCombine::Max),
    //        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
    //        ColliderDensity(2.0),
    //        PlayerGraphicsBundle {
    //            graphics_bundle: GraphicsBundle::new(
    //                asset_server.load("sprites/hehe.png"),
    //                &mut texture_atlas_layouts,
    //                UVec2::new(10, 20),
    //                6,
    //                3,
    //                Vec2::new(0., -10.)
    //            ), 
    //            state: GraphicsState::Falling,
    //            animation_list: AnimationList(HashMap::from([
    //                    (GraphicsState::Idle, StateAnimation::new_timer(12, 17, 450)),
    //                    (GraphicsState::Running, StateAnimation::new_timer(0, 5, 300)),
    //                    (GraphicsState::Jumping, StateAnimation::new_velocity_list(6, 11, vec![
    //                        100.,
    //                        200.,
    //                        300.,
    //                        400.,
    //                        500.,
    //                    ])),
    //                    (GraphicsState::Falling, StateAnimation::new_velocity_list(11, 6, vec![
    //                        -10.,
    //                        -100.,
    //                        -220.,
    //                        -230.,
    //                        -250.,
    //                    ])),
    //            ])),
    //            //state_animation: StateAnimation {
    //            //    idle_indexs: (12, 17),
    //            //    running_indexs: (0,5),
    //            //    jumping_indexs: (6,11),
    //            //    falling_indexs: (11, 6)
    //            //},
    //            //state_animation_speed: StateAnimationSpeed {
    //            //    idle_timer: Timer::new(Duration::from_millis(550), TimerMode::Repeating),
    //            //    running_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
    //            //}
    //        },
    //        //SpriteBundle {
    //        //    texture: hehe_texture_handle,
    //        //    transform: Transform::from_xyz(0., -10., 0.).with_scale(Vec3::splat(2.)),
    //        //    ..default()
    //        //},
    //        //TextureAtlas {
    //        //    layout: hehe_texture_atlas_layout,
    //        //    index: 1
    //        //}
    //       HealthBundle::new(100.) 
    //    ),
    //);
    //commands.spawn(PlatformBundle::new(0., -100., 5000., 10.));
    ////commands.spawn(PlatformBundle::default());
    //for _i in 0..15 {
    //    let mut rng = rand::thread_rng();
    //    let x = rng.gen_range(-250.0..250.0);
    //    let y = rng.gen_range(20.0..1000.0);
    //    let oset = rng.gen_range(-45.0..45.);
    //    if rng.gen_bool(0.5) {
    //        commands.spawn((
    //            PlatformBundle::default().with_location(x,y),
    //            PointLight2d {
    //                radius: 200.0,
    //                intensity: 3.5,
    //                cast_shadows: true,
    //                falloff: 4.5,
    //                color: Color::WHITE,
    //            }
    //        ));
    //    } else {
    //        commands.spawn((
    //            PlatformBundle::default().with_location(x,y),
    //        ));
    //    }
    //    commands.spawn(PlatformBundle::default_wall().with_location(x+oset,y+20.));
    //}
    ////commands.spawn_batch(
    ////    vec![PlatformBundle::default().with_random_location(); 3]
    ////);
    //commands.spawn((
    //    Name::new("Carrot Sword"),
    //    GraphicsBundle::new(
    //        asset_server.load("sprites/carrot.png"),
    //        &mut texture_atlas_layouts,
    //        UVec2::new(5,30),
    //        1,
    //        1,
    //        Vec2::new(100.,0.)
    //    )
    //        .with_anchor(Anchor::BottomRight)
    //        .with_z_index(10.),
    //    ItemBundle::default()
    //        //.with_position(50.,-50.)
    //        .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
    //        .with_use_time(250)
    //        .with_swing_desc(4. * PI / 3., PI / 6., 4.* PI / 3.)
    //));
    //commands.spawn((
    //    Name::new("Strawberry Dagger"),
    //    GraphicsBundle::new(
    //        asset_server.load("sprites/strawberry.png"),
    //        &mut texture_atlas_layouts,
    //        UVec2::new(3,6),
    //        1,
    //        1,
    //        Vec2::new(-100.,0.)
    //    )
    //        .with_anchor(Anchor::BottomRight)
    //        .with_z_index(10.),
    //    ItemBundle::default()
    //        //.with_position(50.,-50.)
    //        .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
    //        .with_use_time(180)
    //        .with_swing_desc(PI/6., PI/12., PI/6.)
    //));
    //commands.spawn((
    //    Name::new("Haha"),
    //    GraphicsBundle::new(
    //        asset_server.load("sprites/haha.png"),
    //        &mut texture_atlas_layouts,
    //        UVec2::new(20,50),
    //        4,
    //        2,
    //        Vec2::new(100., 100.),
    //    ),
    //    CharacterControllerBundle::new(Collider::rectangle(20., 50.)).with_movement(
    //        0.8,
    //        0.8,
    //        RunBundle::default(),
    //        JumpBundle::default(),
    //
    //        FallBundle::default(),
    //        40.,
    //        30),
    //    HealthBundle::new(100.)
    //));
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    character_controler: CharacterControllerBundle,
            //Collider::rectangle(10.0, 10.0),
            //LockedAxes::ROTATION_LOCKED,
    friction: Friction,
    restitution: Restitution,
    collider_density: ColliderDensity,
    //#[sprite_bundle]
    #[sprite_sheet_bundle("sprites/hehe.png", 10, 20, 6, 3, 1, 0, 0)]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player_graphics_bundle: PlayerGraphicsBundle,
    //player_graphics_bundle: PlayerGraphicsBundle,
    #[worldly]
    worldly: Worldly,
    health: HealthBundle
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hehe"),
            player: Player,
            character_controler: CharacterControllerBundle::new(Collider::rectangle(10.0, 20.0)).with_movement(
                0.80,
                0.50,
                RunBundle::new(
                    20.,
                    380.,
                    550,
                    Vec2::new(0.5, 0.5),
                    Vec2::new(0.5, 0.5)
                ),
                JumpBundle::new(
                    800.,
                    200,
                    3,
                    Vec2::new(0., 1.),
                    Vec2::new(0.7, 0.9),
                ),
                FallBundle::new(
                    // TODO: Get rid of initial fall speed??
                    -0.,
                    -800.,
                    550,
                    Vec2::new(0.0, 1.0),
                    Vec2::new(0.50, 1.0)
                ),
                (30.0 as Scalar).to_radians(),
                100,
                ),
                    friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Max),
                    restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                    collider_density: ColliderDensity(2.0),
                    sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
                    player_graphics_bundle: PlayerGraphicsBundle {
                        facing: Facing::default(),
                        state: GraphicsState::Falling,
                        animation_list: AnimationList(HashMap::from([
                                (GraphicsState::Idle, StateAnimation::new_timer(12, 17, 450)),
                                (GraphicsState::Running, StateAnimation::new_timer(0, 5, 300)),
                                (GraphicsState::Jumping, StateAnimation::new_velocity_list(6, 11, vec![
                                                                                           100.,
                                                                                           200.,
                                                                                           300.,
                                                                                           400.,
                                                                                           500.,
                                ])),
                                (GraphicsState::Falling, StateAnimation::new_velocity_list(11, 6, vec![
                                                                                           -10.,
                                                                                           -100.,
                                                                                           -220.,
                                                                                           -230.,
                                                                                           -250.,
                                ])),
                        ])),
                    },
                    worldly: Worldly::default(),
                    health: HealthBundle::new(100.) 
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct HahaBundle {
    name: Name,
    controller: CharacterControllerBundle,
    health: HealthBundle
}

impl Default for HahaBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Haha"),
            controller: CharacterControllerBundle::new(Collider::rectangle(20., 50.)).with_movement(
                0.8,
                0.8,
                RunBundle::default(),
                JumpBundle::default(),

                FallBundle::default(),
                40.,
                30),
            health: HealthBundle::new(100.)
        }
    }
}

#[derive(Bundle, Clone, LdtkIntCell)]
pub struct GreenLightingBundle {
    pub light2d: PointLight2d
}
impl Default for GreenLightingBundle {
    fn default() -> Self {
        Self {
            light2d: PointLight2d {
                radius: 200.0,
                intensity: 1.5,
                cast_shadows: true,
                falloff: 4.5,
                color: Color::linear_rgb(
                    0., 1., 0.2),
            }
        }
    }
}
#[derive(Bundle, Clone, LdtkIntCell)]
pub struct PlatformBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub friction: Friction,
    pub collision_layer: CollisionLayers,
    //pub sprite_bundle: SpriteBundle,
}

impl Default for PlatformBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static, 
            collider: Collider::rectangle(16.0, 16.0),
            friction: Friction::new(0.5).with_static_coefficient(0.),
            collision_layer: CollisionLayers::new(GameLayer::GROUND,[GameLayer::CHARACTER]),
            //sprite_bundle: SpriteBundle {
            //    sprite: Sprite {
            //        color: Color::WHITE,
            //        custom_size: Some(Vec2::new(100., 10.)),
            //        ..default()
            //    },
            //    transform: Transform::from_xyz(0., -100., 0.),    
            //    ..default()
            //},
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
            //sprite_bundle: SpriteBundle {
            //    sprite: Sprite {
            //        color: Color::WHITE,
            //        custom_size: Some(Vec2::new(width, height)),
            //        ..Default::default()
            //    },
            //    transform: Transform::from_xyz(x, y, 0.),    
            //    ..Default::default()
            //},
        }
    }
    pub fn default_wall() -> Self {
        Self::new(0., -100., 10., 30.,)
        //Self {
        //    rigid_body: RigidBody::Static, 
        //    collider: Collider::rectangle(10.0, 30.0),
        //    friction: Friction::ZERO,
        //    sprite_bundle: SpriteBundle {
        //        sprite: Sprite {
        //            color: Color::WHITE,
        //            custom_size: Some(Vec2::new(10., 30.)),
        //            ..Default::default()
        //        },
        //        transform: Transform::from_xyz(0., -100., 0.),    
        //        ..Default::default()
        //    },
        //    light: PointLight::default()
        //}
    }
    //pub fn with_random_location(mut self) -> Self {
    //    let mut rng = rand::thread_rng();
    //    let x = rng.gen_range(-100.0..100.0);
    //    let y = rng.gen_range(-100.0..100.0);
    //    self.sprite_bundle.transform = Transform::from_xyz(
    //        x,y,0.
    //    );
    //    self
    //}
    //pub fn with_location(mut self, x: f32, y: f32) -> Self {
    //    self.sprite_bundle.transform = Transform::from_xyz(
    //        x,y,0.
    //    );
    //    self
    //}
}
