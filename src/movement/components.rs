use std::time::Duration;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;


/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The time a character hangs (aka Cyote Time)
#[derive(Component)]
pub struct HangTime(pub Timer);

/// The acceleration used for character movement.
#[derive(Component, Reflect)]
pub struct MovementAcceleration(pub Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Reflect)]
pub struct MovementDampingFactor(pub Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct JumpImpulse(pub Scalar);

/// The strength of a jump.
#[derive(Component, Reflect)]
pub struct FallGravityScale(pub Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component, Reflect)]
pub struct MaxSlopeAngle(pub Scalar);

#[derive(Component, Reflect)]
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
    pub jump_fall_counter: JumpFallCounter,
    pub movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    pub acceleration: MovementAcceleration,
    pub damping: MovementDampingFactor,
    pub jump_impulse: JumpImpulse,
    pub max_slope_angle: MaxSlopeAngle,
    pub fall_gravity_scale: FallGravityScale,
    pub gravity_scale: GravityScale,
    pub hang_timer: HangTime,
    pub max_jump_count: MaxJumpCount
}

impl MovementBundle {
    pub fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
        fall_gravity_scale: Scalar,
        hang_duration: u64,
        max_jump_count: i32
    ) -> Self {
        let mut hang_timer = Timer::default();
        hang_timer.set_mode(TimerMode::Once);
        hang_timer.set_duration(Duration::from_millis(hang_duration));
        hang_timer.pause();
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            fall_gravity_scale: FallGravityScale(fall_gravity_scale),
            gravity_scale: GravityScale(1.),
            hang_timer: HangTime(hang_timer),
            max_jump_count: MaxJumpCount(max_jump_count)
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(
            30.0, 
            0.9, 
            7.0, 
            PI * 0.45,
            1.5,
            100,
            2
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
            jump_fall_counter: JumpFallCounter(0),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
        fall_gravity_scale: Scalar,
        hang_duration: u64,
        max_jump_count: i32
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle, fall_gravity_scale, hang_duration, max_jump_count);
        self
    }
}

