use std::{collections::HashMap, f32::consts::PI, time::Duration};

use avian2d::{collision::Collider, prelude::Sensor};
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

use crate::Facing;

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
        (rotation - self.start_angle_bounded()) / (self.end_angle_bounded() - self.start_angle_bounded())
    }
    pub fn swing_range(&self) -> f32 {
        self.end_angle_bounded() - self.start_angle_bounded()
    }
    pub fn swing_direction(&self) -> f32 {
        if self.start_angle_bounded() > self.end_angle_bounded() {
            -1.
        } else {
            1.
        }
    }
    pub fn rest_angle_bounded(&self) -> f32 {
        self.rest_angle % (2. * PI)
    }
    /// Returns the start angle between [0, 2.*PI]
    pub fn start_angle_bounded(&self) -> f32 {
        self.start_angle % (2. * PI)
    }
    pub fn end_angle_bounded(&self) -> f32 {
        self.end_angle % (2. * PI)
    }
}

#[derive(Component)]
pub struct UseAccel(pub CubicSegment<Vec2>);

#[derive(Component)]
pub struct UseTime(pub Timer);

#[derive(Bundle)]
pub struct SwingBundle {
    pub use_accel: UseAccel,
    pub use_time: UseTime,
    pub swing_desc: SwingDesc
}

#[derive(Event)]
pub enum ItemAction {
    Use,
    Eat,
    Start,
    End,
    Rest
    // Maybe something like
    // UseSecondary
}

#[derive(Bundle, LdtkEntity)]
pub struct ItemBundle {
    pub item: Item,
    //pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub swing_bundle: SwingBundle,
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            item: Item,
            //sprite_bundle: SpriteBundle {
            //    sprite: Sprite {
            //        color: Color::linear_rgb(1., 0.2, 0.),
            //        custom_size: Some(Vec2::new(2.,30.)),
            //        anchor: Anchor::BottomRight,
            //        ..Default::default()
            //    },
            //    ..Default::default()
            //},
            collider: Collider::rectangle(5., 30.),
            sensor: Sensor,
            swing_bundle: SwingBundle {
                swing_desc: SwingDesc {
                    rest_angle: (4. / 3.) * PI,
                    start_angle: PI / 3.,
                    end_angle: PI,
                },
                use_accel: UseAccel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.0))),
                use_time: UseTime(Timer::new(Duration::from_millis(10000), TimerMode::Once)),
            },
        }
    }
}

impl ItemBundle {
    //pub fn with_position(mut self, x: f32, y: f32) -> Self {
    //    self.sprite_bundle.transform.translation = Vec3::new(x, y, 0.);
    //    self
    //}
    pub fn with_swing_desc(mut self, rest_angle: f32, start_angle: f32, end_angle: f32) -> Self {
        self.swing_bundle.swing_desc = SwingDesc {rest_angle, start_angle, end_angle};
        self
    }
    pub fn with_use_accel(mut self, curve: CubicSegment<Vec2>) -> Self {
        self.swing_bundle.use_accel.0 = curve;
        self
    }
    pub fn with_use_time(mut self, use_time: u64) -> Self {
        self.swing_bundle.use_time = UseTime(Timer::new(Duration::from_millis(use_time), TimerMode::Once));
        self
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct CarrotSword {
    pub name: Name,
    pub item_bundle: ItemBundle,
    #[sprite_bundle("sprites/carrot.png")]
    pub sprite_sheet: SpriteBundle
}
impl Default for CarrotSword {
    fn default() -> Self {
        Self {
            name: Name::new("Carrot Sword"),
            item_bundle: ItemBundle::default()
                //.with_position(50.,-50.)
                .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
                .with_use_time(250)
                .with_swing_desc(4. * PI / 3., PI / 6., 4.* PI / 3.),
            sprite_sheet: SpriteBundle::default()
        }
    }
}
