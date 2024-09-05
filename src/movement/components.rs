use std::time::Duration;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_reflect::Reflect;

/// Collision Layers
#[derive(PhysicsLayer)]
pub enum GameLayer {
    NOTHING,
    CHARACTER,
    GROUND,
}
/// An event sent for a movement input action.
#[derive(Event)]
pub struct MovementEvent {
    pub entity: Entity,
    pub action: MovementAction,
}

impl MovementEvent {
    pub fn new(entity: Entity, action: MovementAction) -> Self {
        Self { entity, action }
    }
}

pub enum MovementAction {
    RunRight,
    RunLeft,
    RunEnd,
    JumpStart,
    JumpEnd,
    Fall,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The time a character hangs (aka Cyote Time)
#[derive(Component, Reflect)]
pub struct HangTime(pub Timer);

/// The acceleration used for character movement.
#[derive(Component, Reflect)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
pub struct GroundMovementDampingFactor(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
pub struct AirMovementDampingFactor(pub Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct JumpImpulse(pub Scalar);

/// The acceleration curve of a jump
#[derive(Component, Reflect)]
pub struct JumpCurve(pub CubicSegment<Vec2>);

/// The length of a jump
#[derive(Component, Reflect)]
pub struct JumpTimer(pub Timer);

/// The hieght of a jump
#[derive(Component, Reflect)]
pub struct JumpHeight(pub Scalar);

#[derive(Component, PartialEq, Debug)]
pub enum JumpFallState {
    Jumping,
    Falling,
    Idle,
}

#[derive(Component, PartialEq, Debug)]
pub enum MoveState {
    Running(f32),
    Dashing,
    Idle,
}

/// The initial move velocity
#[derive(Component, Reflect)]
pub struct InitialRunSpeed(pub f32);
/// The maximum move velocity
#[derive(Component, Reflect)]
pub struct MaxRunSpeed(pub f32);
/// The duration of move acceleration
#[derive(Component, Reflect)]
pub struct RunTimer(pub Timer);

/// The run acceleration curve
#[derive(Component, Reflect)]
pub struct RunCurve(pub CubicSegment<Vec2>);
/// A bundle containing all nessacary Run Components
#[derive(Bundle)]
pub struct RunBundle {
    pub initial_run_speed: InitialRunSpeed,
    pub max_run_speed: MaxRunSpeed,
    pub run_timer: RunTimer,
    pub run_curve: RunCurve,
    pub move_state: MoveState,
}

impl RunBundle {
    pub fn new(
        initial_run_speed: f32,
        max_run_speed: f32,
        duration: u64,
        curve_control1: Vec2,
        curve_control2: Vec2,
    ) -> Self {
        Self {
            initial_run_speed: InitialRunSpeed(initial_run_speed),
            max_run_speed: MaxRunSpeed(max_run_speed),
            run_timer: RunTimer(Timer::new(Duration::from_millis(duration), TimerMode::Once)),
            run_curve: RunCurve(CubicSegment::new_bezier(curve_control1, curve_control2)),
            move_state: MoveState::Idle,
        }
    }
}

impl Default for RunBundle {
    fn default() -> Self {
        Self::new(5., 30., 250, Vec2::new(0.25, 0.1), Vec2::new(0.25, 1.))
    }
}
/// A bundle contataining all nessacary Jump Components
#[derive(Bundle)]
pub struct JumpBundle {
    pub jump_height: JumpHeight,
    pub jump_timer: JumpTimer,
    pub jump_curve: JumpCurve,
    pub max_jump_count: MaxJumpCount,
    pub jump_fall_counter: JumpFallCounter,
    pub jump_fall_state: JumpFallState,
}

impl JumpBundle {
    pub fn new(
        height: f32,
        duration: u64,
        max_jump_count: i32,
        curve_control1: Vec2,
        curve_control2: Vec2,
    ) -> Self {
        Self {
            jump_height: JumpHeight(height),
            jump_timer: JumpTimer(Timer::new(Duration::from_millis(duration), TimerMode::Once)),
            jump_curve: JumpCurve(CubicSegment::new_bezier(curve_control1, curve_control2)),
            max_jump_count: MaxJumpCount(max_jump_count),
            jump_fall_counter: JumpFallCounter(0),
            jump_fall_state: JumpFallState::Falling,
        }
    }
}

impl Default for JumpBundle {
    fn default() -> Self {
        Self::new(100., 250, 1, Vec2::new(0.25, 0.1), Vec2::new(0.25, 1.))
        //Self {
        //    jump_height: JumpHeight(100.),
        //    jump_timer: JumpTimer(Timer::new(Duration::from_millis(250), TimerMode::Once)),
        //    jump_curve: JumpCurve(CubicSegment::new_bezier(Vec2::new(0.25,0.1), Vec2::new(0.25, 1.))),
        //    max_jump_count: MaxJumpCount(1),
        //    jump_fall_counter: JumpFallCounter(0),
        //    jump_fall_state: JumpFallState::Falling
        //}
    }
}

/// The initial move velocity
#[derive(Component, Reflect)]
pub struct InitialFallSpeed(pub f32);
/// The maximum move velocity
#[derive(Component, Reflect)]
pub struct MaxFallSpeed(pub f32);
/// The duration of move acceleration
#[derive(Component, Reflect)]
pub struct FallTimer(pub Timer);

/// The run acceleration curve
#[derive(Component, Reflect)]
pub struct FallCurve(pub CubicSegment<Vec2>);
/// A bundle containing all nessacary Fall Components
#[derive(Bundle)]
pub struct FallBundle {
    pub initial_fall_speed: InitialFallSpeed,
    pub max_fall_speed: MaxFallSpeed,
    pub fall_timer: FallTimer,
    pub fall_curve: FallCurve,
}
impl FallBundle {
    pub fn new(
        initial_fall_speed: f32,
        max_fall_speed: f32,
        duration: u64,
        curve_control1: Vec2,
        curve_control2: Vec2,
    ) -> Self {
        Self {
            initial_fall_speed: InitialFallSpeed(initial_fall_speed),
            max_fall_speed: MaxFallSpeed(max_fall_speed),
            fall_timer: FallTimer(Timer::new(Duration::from_millis(duration), TimerMode::Once)),
            fall_curve: FallCurve(CubicSegment::new_bezier(curve_control1, curve_control2)),
        }
    }
}

impl Default for FallBundle {
    fn default() -> Self {
        Self::new(-5., -30., 250, Vec2::new(0.25, 0.1), Vec2::new(0.25, 1.))
    }
}

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct FallGravityScale(pub Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component, Reflect)]
pub struct MaxSlopeAngle(pub Scalar);

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct MaxJumpCount(pub i32);

#[derive(Component, Reflect)]
pub struct JumpFallCounter(pub i32);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    pub character_controller: CharacterController,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub ground_caster: ShapeCaster,
    pub locked_axes: LockedAxes,
    pub collision_layer: CollisionLayers,
    //pub jump_fall_counter: JumpFallCounter,
    pub movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    pub ground_damping: GroundMovementDampingFactor,
    pub air_damping: AirMovementDampingFactor,
    //pub jump_impulse: JumpImpulse,
    pub run_bundle: RunBundle,
    pub jump_bundle: JumpBundle,
    pub fall_bundle: FallBundle,
    pub max_slope_angle: MaxSlopeAngle,
    //pub fall_gravity_scale: FallGravityScale,
    pub gravity_scale: GravityScale,
    pub hang_timer: HangTime,
    //pub max_jump_count: MaxJumpCount
}

impl MovementBundle {
    pub fn new(
        ground_damping: Scalar,
        air_damping: Scalar,
        //jump_impulse: Scalar,
        run_bundle: RunBundle,
        jump_bundle: JumpBundle,
        fall_bundle: FallBundle,
        max_slope_angle: Scalar,
        //fall_gravity_scale: Scalar,
        hang_duration: u64,
        //max_jump_count: i32
    ) -> Self {
        let mut hang_timer = Timer::default();
        hang_timer.set_mode(TimerMode::Once);
        hang_timer.set_duration(Duration::from_millis(hang_duration));
        hang_timer.pause();
        Self {
            ground_damping: GroundMovementDampingFactor(ground_damping),
            air_damping: AirMovementDampingFactor(air_damping),
            //jump_impulse: JumpImpulse(jump_impulse),
            run_bundle,
            jump_bundle,
            fall_bundle,
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            //fall_gravity_scale: FallGravityScale(fall_gravity_scale),
            gravity_scale: GravityScale(0.),
            hang_timer: HangTime(hang_timer),
            //max_jump_count: MaxJumpCount(max_jump_count)
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(
            0.9,
            0.9,
            RunBundle::default(),
            JumpBundle::default(),
            FallBundle::default(),
            PI * 0.45,
            100,
        )
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_time_of_impact(20.)
                //TODO: Find a better max hit number, may be a problem with more rigid bodies
                .with_max_hits(30),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_layer: CollisionLayers::new(GameLayer::CHARACTER, [GameLayer::GROUND]),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        ground_damping: Scalar,
        air_damping: Scalar,
        run_bundle: RunBundle,
        jump_bundle: JumpBundle,
        fall_bundle: FallBundle,
        max_slope_angle: Scalar,
        hang_duration: u64,
    ) -> Self {
        self.movement = MovementBundle::new(
            ground_damping,
            air_damping,
            run_bundle,
            jump_bundle,
            fall_bundle,
            max_slope_angle,
            hang_duration,
        );
        self
    }
}
