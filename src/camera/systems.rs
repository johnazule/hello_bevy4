use crate::{PhysicsSet, Player};
use bevy::prelude::*;

use super::prelude::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            camera_follow_player
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    for mut camera_transform in camera_query.iter_mut() {
        player_query.iter().for_each(|player_transform| {
            camera_transform.translation = camera_transform
                .translation
                .lerp(player_transform.translation, 0.8)
                + Vec3::new(0., 40., 0.);
        });
    }
}
