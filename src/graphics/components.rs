use bevy::{prelude::*, sprite::Anchor};

/// The direction the entity is facing
#[derive(Component, Clone, Debug)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Bundle)]
pub struct GraphicsBundle {
    pub sprite: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub facing: Facing
}

impl GraphicsBundle {
    pub fn new(
        handle: Handle<Image>,
        mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_size: UVec2,
        tile_columns: u32,
        tile_rows: u32,
        origin: Vec2,
    ) -> Self {
        let atlas_layout = TextureAtlasLayout::from_grid(tile_size, tile_columns, tile_rows, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(atlas_layout);
        Self {
            sprite: SpriteBundle {
                texture: handle,
                transform: Transform::from_translation(origin.extend(0.)).with_scale(Vec3::splat(2.)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1
            },
            facing: Facing::Right
        }
    }
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.sprite.sprite.anchor = anchor;
        self
    }
}
