use std::f32::consts::PI;

use avian2d::collision::Collider;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Equipped;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct InUse {
    pub use_percent: f32
}

#[derive(Component)]
pub struct SwingDesc {
    pub rest_angle: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}

impl SwingDesc {
    pub fn use_percent(&self, rotation: f32) -> f32 {
        (rotation - self.start_angle) / (self.end_angle - self.start_angle)
    }
}

#[derive(Component)]
pub struct UseAccel {
    pub velc_function: fn(f32) -> f32,
}

#[derive(Bundle)]
pub struct SwingBundle {
    pub use_accel: UseAccel,
    pub swing_desc: SwingDesc
}

#[derive(Event)]
pub enum ItemAction {
    Use,
    Eat,
    // Maybe something like
    // UseSecondary
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item: Item,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub swing_bundle: SwingBundle
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            item: Item,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::linear_rgb(1., 0.2, 0.),
                    custom_size: Some(Vec2::new(2.,30.)),
                    anchor: Anchor::BottomRight,
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider::rectangle(2., 30.),
            swing_bundle: SwingBundle {
                swing_desc: SwingDesc {
                    rest_angle: PI / 3.,
                    start_angle: PI,
                    end_angle: 0.,
                },
                use_accel: UseAccel { velc_function: |percent| percent}
            }
        }
    }
}

impl ItemBundle {
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.sprite_bundle.transform.translation = Vec3::new(x, y, 0.);
        self
    }
    pub fn with_swing_desc(mut self, rest_angle: f32, start_angle: f32, end_angle: f32) -> Self {
        self.swing_bundle.swing_desc = SwingDesc {rest_angle, start_angle, end_angle};
        self
    }
    pub fn with_use_accel(mut self, function: fn(f32) -> f32) -> Self {
        self.swing_bundle.use_accel.velc_function = function;
        self
    }
}
