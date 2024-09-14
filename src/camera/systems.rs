use crate::{PhysicsSet, Player};
use bevy::prelude::*;
use bevy_light_2d::light::AmbientLight2d;

use super::components::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_camera).add_systems(
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

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        AmbientLight2d {
            brightness: 0.2,
            color: Color::linear_rgb(1.00, 0.8, 0.5),
        },
        PlayerCamera,
    ));
}
