use std::f32::consts::PI;

use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_light_2d::plugin::Light2dPlugin;

use crate::Facing;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                flip_sprite,
                state_machine
            ))
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

fn state_machine(
    mut query: Query<(&LinearVelocity, &mut TextureAtlas)>
) {
    for (linear_velocity, mut texture_atlas) in query.iter_mut() {
        if linear_velocity.y > 1. {
            info!("Jumping");
        } else if linear_velocity.y < -1. {
            info!("Falling");
        }
        //info!("Linear Velocity: {:?}", linear_velocity);
        //info!("Sprite Index: {}", texture_atlas.index);
    }
}
