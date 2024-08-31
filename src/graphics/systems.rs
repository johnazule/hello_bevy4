use std::f32::consts::PI;

use avian2d::prelude::LinearVelocity;
use bevy::{ecs::entity, prelude::*};
use bevy_light_2d::plugin::Light2dPlugin;

use crate::{setup, AnimTimer, AnimationList, Facing, GraphicsBundle, Grounded, Health, Healthbar, StateAnimation, StateChange};

use super::prelude::GraphicsState;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StateChange>()
            .add_systems(Update, (
                flip_sprite,
                state_machine,
                set_state,
                set_sprite_from_state
            ).chain())
            .insert_resource(Msaa::Off)
            .add_plugins(Light2dPlugin);
    }
}

fn flip_sprite(mut query: Query<(&mut Transform, &mut Sprite, &Facing, &Name)>) {
   for (mut transform, mut sprite, facing, name) in query.iter_mut() {
       //info!("{} is facing {:?}", name, facing);
       match facing {
           Facing::Right => {
               //transform.rotation= Quat::default();
               sprite.flip_x = false;
           }
           Facing::Left => {
               //transform.rotation= Quat::from_rotation_x(PI);
               sprite.flip_x = true;
           }
       }
   } 
}

fn set_state(
    mut query: Query<(&mut GraphicsState)>,
    mut state_change_event_reader: EventReader<StateChange>
) {
    for graphics_state_change in state_change_event_reader.read() {
        //info!("Here!!");
        let graphics_state_result = query.get_mut(graphics_state_change.entity);
        if graphics_state_result.is_ok() {
            let new_state = graphics_state_change.state.clone();
            let debug_state = new_state.clone();
            *graphics_state_result.unwrap() = new_state;
            //info!("The state ({:?}) has been set!!", debug_state);
        }
    }
}
fn state_machine(
    query: Query<(&LinearVelocity, Entity, &GraphicsState, Has<Grounded>)>,
    mut state_change_event_writer: EventWriter<StateChange>
) {
    for (linear_velocity, entity, state, grounded) in query.iter() {
        //info!("Linear Velocity (y): {}", linear_velocity.y);
        if linear_velocity.y > 1. {
            if state.is_not_state(&GraphicsState::Jumping) {
                state_change_event_writer.send(StateChange {
                    state: GraphicsState::Jumping,
                    entity
                });
            }
        } else if linear_velocity.y < -0.5 {
            if state.is_not_state(&GraphicsState::Falling) {
                state_change_event_writer.send(StateChange {
                    state: GraphicsState::Falling,
                    entity
                });
            }
        } else if linear_velocity.x.abs() > 1. {
            if state.is_not_state(&GraphicsState::Running) {
                state_change_event_writer.send(StateChange {
                    state: GraphicsState::Running,
                    entity
                });
            }
        } else if !grounded {
            if state.is_not_state(&GraphicsState::Idle) {
                state_change_event_writer.send(StateChange {
                    state: GraphicsState::Falling,
                    entity
                });
            }
        } else {
            if state.is_not_state(&GraphicsState::Idle) {
                state_change_event_writer.send(StateChange {
                    state: GraphicsState::Idle,
                    entity
                });
            }
        }
        //info!("Linear Velocity: {:?}", linear_velocity);
        //info!("Sprite Index: {}", texture_atlas.index);
    }
}


fn set_sprite_from_state(
    mut query: Query<(&GraphicsState, &LinearVelocity, &mut AnimationList, &mut TextureAtlas)>,
    time: Res<Time>
) {
    for (graphics_state, linear_velocity, mut animation_list, mut texture_atlas) in query.iter_mut() {
        match animation_list.0.get_mut(graphics_state) {
            Some(state_animation) => {
                if state_animation.has_velocity_list() {
                    //info!("Velocity: {}", linear_velocity.y);
                    texture_atlas.index = state_animation.frame_from_velocity(linear_velocity.y);
                } else {
                    state_animation.anim_timer.tick_timer(time.delta());
                    texture_atlas.index = state_animation.frame_from_percent();
                }
            },
            None => return
        }
        //match *graphics_state {
        //    GraphicsState::Jumping => {
        //        texture_atlas.index = state_animation.jumping_indexs.0;
        //    }
        //    GraphicsState::Falling => {
        //        texture_atlas.index = state_animation.falling_indexs.0;
        //    }
        //    GraphicsState::Running => {
        //        state_animation_speed.running_timer.tick(time.delta());
        //        texture_atlas.index = state_animation.running_frame_from_percent(state_animation_speed.running_timer.fraction());
        //    }
        //    GraphicsState::Idle => {
        //        state_animation_speed.idle_timer.tick(time.delta());
        //        let new_index = state_animation.idle_frame_from_percent(state_animation_speed.idle_timer.fraction());
        //        texture_atlas.index = new_index; 
        //    }
        //
        //} 
    }
}
