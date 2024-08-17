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
                movement,
                thwampy_gravity,
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
        (Entity, &ShapeHits, &Rotation, &mut HangTime, &mut JumpFallCounter, Option<&MaxSlopeAngle>, Has<Grounded>),
        With<CharacterController>,
    >,
) {
    for (
        entity,
        hits,
        rotation,
        mut hang_time,
        mut jump_fall_counter,
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
            if !is_already_grounded {
                jump_fall_counter.0 = 0;
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

pub fn thwampy_gravity(
    mut query: Query<(
        &mut GravityScale,
        &FallGravityScale,
        &LinearVelocity,
        Has<Grounded>
    )>
) {
    for (mut gravity, fall_gravity, linear_velocity, _grounded) in query.iter_mut() {
        if linear_velocity.y.signum() <= 0. {
            gravity.0 = fall_gravity.0;
        } else {
            gravity.0 = 1.0;
        }
    }
}
/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        &HangTime,
        &mut JumpFallCounter,
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
                movement_acceleration,
                jump_impulse,
                mut linear_velocity,
                hang_time,
                mut jump_fall_counter,
                max_jump_counter,
                mut facing,
                is_grounded
            ) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    if *direction > 0. {
                        *facing = Facing::Right;
                    } else {
                        *facing = Facing::Left;
                    }
                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
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
                        linear_velocity.y = jump_impulse.0;
                    }
                    //info!("Grounded:\t\t{}", is_grounded);
                    //info!("Has Jumps Left:\t{}", has_jumps_left);
                    //info!("Is Base Jump:\t\t{}", is_base_jump);
                    //info!("Still Hanging:\t{}", still_hanging);
                    //info!("Jump Counter:\t\t{}", jump_fall_counter.0);
                    //info!("Can Jump:\t\t{}\n", can_jump);

                }
            }
        }
    }
}

/// Slows down movement in the X direction.
pub fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
