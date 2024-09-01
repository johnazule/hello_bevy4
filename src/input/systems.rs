use bevy::prelude::*;
use avian2d::math::*;

use crate::{movement::prelude::*, Equipped, InUse, Item, ItemAction, Player};

pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                keyboard_input,
                //gamepad_input
        ));
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
pub fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementEvent>,
    mut item_event_writer: EventWriter<ItemAction>,
    equipped_item: Query<(&Equipped, Has<InUse>), With<Item>>,
    player_query: Query<Entity, (With<Player>, Without<Item>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    let player_entity_result = player_query.get_single();
    if let Ok(player_entity) = player_entity_result {
        let left_pressed = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right_pressed = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
        let jump_pressed = keyboard_input.any_just_pressed([KeyCode::Space]);
        let fall_pressed = keyboard_input.any_just_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

        let left_released = keyboard_input.any_just_released([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right_released = keyboard_input.any_just_released([KeyCode::KeyD, KeyCode::ArrowRight]);
        let jump_released = keyboard_input.any_just_released([KeyCode::Space]);

        if left_pressed && right_pressed {
            movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::RunEnd));
        } else {
            if left_pressed {
                movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::RunLeft));
            }
            if right_pressed {
                movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::RunRight));
            }
        }
        
        if (left_released || right_released) && !(left_pressed || right_pressed) {
            movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::RunEnd));
        }

        if jump_pressed {
            movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::JumpStart));
        }
        if jump_released {
            movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::JumpEnd));
        }
        if fall_pressed {
            movement_event_writer.send(MovementEvent::new(player_entity, MovementAction::Fall));
        }
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            item_event_writer.send(ItemAction::Start);
        }
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            item_event_writer.send(ItemAction::End);
        }
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            item_event_writer.send(ItemAction::Rest);
        }

        if mouse_input.pressed(MouseButton::Left) {
            let Ok((_equipped, in_use)) = equipped_item.get_single() else {
                return;
            };
            if !in_use {
                item_event_writer.send(ItemAction::Use);
            }
        }
    } else {

    }
}

// Sends [`MovementAction`] events based on gamepad input.
//pub fn gamepad_input(
//    mut movement_event_writer: EventWriter<MovementAction>,
//    gamepads: Res<Gamepads>,
//    axes: Res<Axis<GamepadAxis>>,
//    buttons: Res<ButtonInput<GamepadButton>>,
//) {
//    for gamepad in gamepads.iter() {
//        let axis_lx = GamepadAxis {
//            gamepad,
//            axis_type: GamepadAxisType::LeftStickX,
//        };
//
//        if let Some(x) = axes.get(axis_lx) {
//            movement_event_writer.send(MovementAction::Move(x as Scalar));
//        }
//
//        let jump_button = GamepadButton {
//            gamepad,
//            button_type: GamepadButtonType::South,
//        };
//
//        if buttons.just_pressed(jump_button) {
//            movement_event_writer.send(MovementAction::Jump);
//        }
//    }
//}
