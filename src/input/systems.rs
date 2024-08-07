use bevy::prelude::*;
use avian2d::math::*;

use crate::{movement::prelude::*, Equipped, InUse, Item, ItemAction};

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
    mut movement_event_writer: EventWriter<MovementAction>,
    mut item_event_writer: EventWriter<ItemAction>,
    equipped_item: Query<(&Equipped, Has<InUse>), With<Item>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_event_writer.send(MovementAction::Jump);
    }

    if mouse_input.pressed(MouseButton::Left) {
        let Ok((_equipped, in_use)) = equipped_item.get_single() else {
            return;
        };
        if !in_use {
            item_event_writer.send(ItemAction::Use);
        }
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
