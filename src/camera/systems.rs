use bevy::prelude::*;
use crate::{Player, PhysicsSet};

use super::prelude::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_player.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate));
    }
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>
) {
    for (mut camera_transform) in camera_query.iter_mut() {
        for (player_transform) in player_query.iter() {
            camera_transform.translation = camera_transform.translation.lerp(player_transform.translation, 0.8) + Vec3::new(0., 40., 0.);
        }
    }
}
