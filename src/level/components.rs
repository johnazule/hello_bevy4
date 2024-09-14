use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::GameLayer;

#[derive(Bundle, Clone, LdtkIntCell)]
pub struct PlatformBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub friction: Friction,
    pub collision_layer: CollisionLayers,
}

impl Default for PlatformBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(16.0, 16.0),
            friction: Friction::new(0.5).with_static_coefficient(0.),
            collision_layer: CollisionLayers::new(GameLayer::GROUND, [GameLayer::CHARACTER]),
        }
    }
}
