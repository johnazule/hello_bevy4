use avian2d::collision::Collider;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Equipped;

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
}

impl Default for ItemBundle {
    fn default() -> Self {
        Self {
            item: Item,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::linear_rgb(1., 0.2, 0.),
                    custom_size: Some(Vec2::new(2.,30.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider::rectangle(2., 30.),
        }
    }
}

impl ItemBundle {
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.sprite_bundle.transform.translation = Vec3::new(x, y, 0.);
        self
    }
}
