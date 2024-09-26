use avian2d::math::Scalar;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::graphics::prelude::*;
use crate::health_damage::prelude::*;
use crate::movement::prelude::*;
use crate::InteractorRange;

#[derive(Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    name: Name,
    player: Player,
    character_controler: CharacterControllerBundle,
    friction: Friction,
    restitution: Restitution,
    collider_density: ColliderDensity,
    #[sprite_sheet_bundle("sprites/hehe.png", 10, 20, 6, 3, 1, 0, 0)]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    player_graphics_bundle: PlayerGraphicsBundle,
    #[worldly]
    worldly: Worldly,
    health: HealthBundle,
    interactor: InteractorRange,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Hehe"),
            player: Player,
            character_controler: CharacterControllerBundle::new(Collider::rectangle(10.0, 20.0))
                .with_movement(
                    0.80,
                    0.50,
                    RunBundle::new(20., 380., 550, Vec2::new(0.5, 0.5), Vec2::new(0.5, 0.5)),
                    JumpBundle::new(800., 200, 3, Vec2::new(0., 1.), Vec2::new(0.7, 0.9)),
                    FallBundle::new(
                        // TODO: Get rid of initial fall speed??
                        -0.,
                        -800.,
                        550,
                        Vec2::new(0.0, 1.0),
                        Vec2::new(0.50, 1.0),
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
                animation_list: AnimationList(std::collections::HashMap::from([
                    (GraphicsState::Idle, StateAnimation::new_timer(12, 17, 450)),
                    (GraphicsState::Running, StateAnimation::new_timer(0, 5, 300)),
                    (
                        GraphicsState::Jumping,
                        StateAnimation::new_velocity_list(
                            6,
                            11,
                            vec![100., 200., 300., 400., 500.],
                        ),
                    ),
                    (
                        GraphicsState::Falling,
                        StateAnimation::new_velocity_list(
                            11,
                            6,
                            vec![-10., -100., -220., -230., -250.],
                        ),
                    ),
                ])),
            },
            worldly: Worldly::default(),
            health: HealthBundle::new(100., [[0.25, 0.1], [0.25, 1.]], 2800)
                .with_current_health(50.),
            interactor: InteractorRange(100.),
        }
    }
}
