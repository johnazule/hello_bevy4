use std::ops::DerefMut;

use avian2d::{math::*, prelude::*};
use bevy::{ecs::query::Has, prelude::*};

use crate::{Facing, Player};

use super::components::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                update_grounded,
                movement_validation,
                jump_fall,
                run,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

/// Updates the [`Grounded`] status for character controllers.
pub fn update_grounded(
    mut commands: Commands,
    time: Res<Time>,
    grounds: Query<Option<&RigidBody>, (With<Collider>, Without<Player>)>,
    mut query: Query<
        (
            Entity,
            &ShapeHits,
            &Rotation,
            &mut HangTime,
            &mut JumpFallCounter,
            &mut JumpTimer,
            &mut FallTimer,
            &mut JumpFallState,
            Option<&MaxSlopeAngle>,
            Has<Grounded>
        ),
        With<CharacterController>,
    >,
) {
    for (
        entity,
        hits,
        rotation,
        mut hang_time,
        mut jump_fall_counter,
        mut jump_timer,
        mut fall_timer,
        mut jump_fall_state,
        max_slope_angle,
        is_already_grounded
    ) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let mut rigid_hits = hits.iter().filter(|hit| {
            grounds.get(hit.entity).unwrap().is_some()
        });
        let is_grounded = rigid_hits.any(|hit| {
        //let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            //info!("This fucker is on the ground");
            commands.entity(entity).insert(Grounded);
            jump_fall_counter.0 = 0;
            if !is_already_grounded {
                jump_timer.0.reset();
                fall_timer.0.reset();
                *jump_fall_state = JumpFallState::Idle;
                hang_time.0.pause();
                hang_time.0.reset();
            }
        } else {
            // Having to tick timers manually is stupid. Like why? Seriously if I have 100 timers, I
            // have to tick each individually? Be so for real
            hang_time.0.tick(time.delta());
            if is_already_grounded {
                hang_time.0.unpause();
            }
            commands.entity(entity).remove::<Grounded>();
        }
        //info!("Is Grounded:\t{}", is_grounded);
    }
}

pub fn jump_fall(
    mut query: Query<(
        &mut LinearVelocity,
        &mut JumpTimer,
        &mut JumpFallState,
        &JumpHeight,
        &JumpCurve,
        &mut FallTimer,
        &InitialFallSpeed,
        &MaxFallSpeed,
        &FallCurve,
    )>,
    time: Res<Time>
) {
    for (
        mut linear_velocity,
        mut jump_timer,
        mut jump_fall_state,
        jump_height,
        jump_curve,
        mut fall_timer,
        initial_fall_speed,
        max_fall_speed,
        fall_curve
    ) in query.iter_mut() {
        info!("Jump Fall State: {:?}", jump_fall_state);
        match *jump_fall_state {
            JumpFallState::Jumping => {
                jump_timer.0.tick(time.delta());
                if jump_timer.0.finished() {
                    *jump_fall_state = JumpFallState::Falling;
                    return;
                }
                // TODO: Jump Curve should goto zero, not accelerate (duh)
                linear_velocity.y = jump_height.0 * jump_curve.0.ease(jump_timer.0.fraction());
            },
            JumpFallState::Falling => {
                jump_timer.0.reset();
                fall_timer.0.tick(time.delta());
                let new_velocity = initial_fall_speed.0 + max_fall_speed.0 * fall_curve.0.ease(fall_timer.0.fraction());
                linear_velocity.y = linear_velocity.y.lerp(new_velocity, 0.1);
            },
            JumpFallState::Idle => {}
        }
    }
}

pub fn run (
    mut query: Query<(
        &mut LinearVelocity,
        &mut MoveState,
        &mut RunTimer,
        &InitialRunSpeed,
        &MaxRunSpeed,
        &RunCurve,
        &MovementDampingFactor
    )>,
    time: Res<Time>
) {
    for (
        mut linear_velocity,
        mut move_state,
        mut run_timer,
        initial_run_speed,
        max_run_speed,
        run_curve,
        damping_factor
    ) in query.iter_mut() {
        match *move_state {
            MoveState::Running(direction) => {
                let mut previous_velocity = direction * (initial_run_speed.0 + max_run_speed.0 * run_curve.0.ease(run_timer.0.fraction()));
                // Simulate Damping
                //previous_velocity *= damping_factor.0;
                run_timer.0.tick(time.delta());
                let new_velocity = direction * (initial_run_speed.0 + max_run_speed.0 * run_curve.0.ease(run_timer.0.fraction()));
                info!("Expected: {}", previous_velocity);
                info!("Current: {}", linear_velocity.x);
                info!("New: {}\n", new_velocity);
                run_timer.0.tick(time.delta());
                linear_velocity.x = new_velocity;
            },
            MoveState::Dashing => {},
            MoveState::Idle => {}
        } 
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement_validation(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &mut JumpFallState,
        &mut MoveState,
        &mut LinearVelocity,
        &HangTime,
        &mut JumpFallCounter,
        &mut RunTimer,
        &MaxJumpCount,
        &mut Facing,
        Has<Grounded>
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_seconds_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (
                mut jump_fall_state,     
                mut move_state,
                //jump_impulse,
                mut linear_velocity,
                hang_time,
                mut jump_fall_counter,
                mut run_timer,
                max_jump_counter,
                mut facing,
                is_grounded
            ) in
            &mut controllers
        {
            match event {
                // TODO: Decide if this should be RunRight or MoveRight
                MovementAction::RunRight => {
                    *facing = Facing::Right;
                    *move_state = MoveState::Running(1.);
                    //linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                },
                MovementAction::RunLeft => {
                    *move_state = MoveState::Running(-1.);
                    *facing = Facing::Left;
                },
                MovementAction::JumpStart => {
                    let has_jumps_left = jump_fall_counter.0 < max_jump_counter.0;
                    let is_base_jump = max_jump_counter.0 == jump_fall_counter.0;
                    let still_hanging = !hang_time.0.finished();
                    let mut can_jump = true;
                    if !has_jumps_left {
                        can_jump = false;
                    }
                    if is_base_jump && !(is_grounded || still_hanging) {
                        can_jump = false;
                    }
                    if can_jump {
                        jump_fall_counter.0 += 1;
                        *jump_fall_state = JumpFallState::Jumping;
                        //linear_velocity.y = jump_impulse.0;
                        //linear_velocity.y += jump_height.0 * jump_curve.0.ease(jump_timer.0.fraction());
                    }
                    info!("Grounded:\t\t{}", is_grounded);
                    info!("Has Jumps Left:\t{}", has_jumps_left);
                    info!("Is Base Jump:\t\t{}", is_base_jump);
                    info!("Still Hanging:\t{}", still_hanging);
                    info!("Jump Counter:\t\t{}", jump_fall_counter.0);
                    info!("Can Jump:\t\t{}\n", can_jump);
                },
                MovementAction::JumpEnd => {
                    if *jump_fall_state == JumpFallState::Jumping {
                        *jump_fall_state = JumpFallState::Falling;
                    }
                },
                MovementAction::RunEnd => {
                    if matches!(*move_state, MoveState::Running(_)) {
                        *move_state = MoveState::Idle;
                        run_timer.0.reset();
                    }
                },
                MovementAction::Fall => {}
            }
        }
    }
}

pub fn apply_movement_damping(
    mut query: Query<(
        &MovementDampingFactor,
        &mut LinearVelocity,
    )>,
) {
    for (
        damping_factor,
        mut linear_velocity,
    ) in &mut query {
        linear_velocity.x *= damping_factor.0;
    } 
}
/// Slows down movement in the X direction.
pub fn apply_air_movement_damping(
    mut query: Query<(
        &MovementDampingFactor,
        &mut LinearVelocity,
        &CollidingEntities
        //&Collision
    ), With<Player>>,
    grounds: Query<Option<&RigidBody>, (With<Collider>, Without<Player>)>,
) {
    for (
        damping_factor,
        mut linear_velocity,
        colliding_entities
    ) in &mut query {
        let mut rigid_hits = colliding_entities.iter().filter(|hit| {
            grounds.get(**hit).unwrap().is_some()
        }).collect::<Vec<&Entity>>();
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        if rigid_hits.is_empty() {
            linear_velocity.x *= damping_factor.0;
        }
    }
}
