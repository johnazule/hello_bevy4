use avian2d::prelude::PhysicsSet;
use bevy::prelude::*;

use crate::{setup, GraphicsBundle, Health, Healthbar};

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
                spawn_health_bars.after(setup)
            ))
            .add_systems(Update, (
                health_bar_follow_entity.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate)
        ));
    }
}
fn spawn_health_bars(
    mut query: Query<(Entity, &Transform), With<Health>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands
) {
    for (entity, transform) in query.iter_mut() {
        info!("Here");
        commands.spawn((
                Healthbar(entity),
                GraphicsBundle::new(
                    asset_server.load("sprites/healthbar.png"),
                    &mut texture_atlas_layouts,
                    UVec2::new(110, 5),
                    1,
                    1,
                    transform.translation.truncate()
                ).with_z_index(500.)
        ));
    }
}

fn health_bar_follow_entity (
    health_query: Query<(Entity, &Handle<Image>), (With<Health>, With<Sprite>)>,
    assets: Res<Assets<Image>>,
    mut transform_parems: ParamSet<(
        TransformHelper,
        Query<&mut Transform, (With<Healthbar>, Without<Health>)>
    )>,
) {
    for (health_entity, health_sprite_handle) in health_query.iter() {
        let player_transform = transform_parems.p0().compute_global_transform(health_entity).unwrap();
        for (mut healthbar_transform) in transform_parems.p1().iter_mut() {
            let health_sprite_height = assets.get(health_sprite_handle).unwrap().size().y as f32;
            healthbar_transform.translation.x = player_transform.translation().x;
            healthbar_transform.translation.y = player_transform.translation().y  + health_sprite_height / 2. + 10.;
        }
    }
}
