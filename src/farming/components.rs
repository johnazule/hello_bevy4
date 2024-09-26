use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Farm;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct PlayerCanInteract;

#[derive(Bundle, LdtkEntity, Default)]
pub struct FarmBundle {
    #[sprite_bundle]
    sprite: SpriteBundle,
    farm: Farm,
}
