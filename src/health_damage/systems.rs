use avian2d::prelude::PhysicsSet;
use bevy::{prelude::*, sprite::Anchor};

use crate::{HealingCurve, HealingTimer, Health, Healthbar, HealthbarBorder, HealthbarFill};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, (spawn_health_bars.after(setup)))
            .add_systems(
                Update,
                (
                    health_bar_follow_entity
                        .after(PhysicsSet::Sync)
                        .before(TransformSystem::TransformPropagate),
                    spawn_health_bars,
                    display_current_health.after(health_bar_follow_entity),
                    hide_full_hp_bar,
                    healing,
                ),
            );
    }
}
fn spawn_health_bars(
    mut query: Query<(Entity, &Transform, &Name), Added<Health>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (entity, transform, name) in query.iter_mut() {
        let mut z_ordered_transform = *transform;
        z_ordered_transform.translation.z = 110.;
        let mut z_ordered_transform_offset = *transform;
        z_ordered_transform_offset.translation.x = -14.;
        commands
            .spawn((
                Healthbar(entity),
                SpatialBundle::from_transform(z_ordered_transform),
            ))
            .with_children(|healthbar_parent| {
                healthbar_parent.spawn((
                    HealthbarBorder,
                    SpriteBundle {
                        texture: asset_server.load("sprites/healthbar.png"),
                        ..default()
                    },
                ));
                healthbar_parent.spawn((
                    HealthbarFill,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::linear_rgb(1., 0., 0.),
                            custom_size: Some(Vec2::new(28., 5.)),
                            anchor: Anchor::CenterLeft,
                            ..default()
                        },
                        transform: Transform::from_xyz(-14., 0., -1.),
                        //transform: *transform,
                        ..default()
                    },
                ));
            });
    }
}

fn health_bar_follow_entity(
    mut bar_query: Query<(&Healthbar, &mut Transform), Without<Health>>,
    health_query: Query<
        (&Transform, &Handle<Image>),
        (With<Sprite>, With<Health>, Without<Healthbar>),
    >,
    assets: Res<Assets<Image>>,
) {
    for (bar_linked_entity, mut bar_transform) in bar_query.iter_mut() {
        let (health_transform, health_sprite_handle) =
            health_query.get(bar_linked_entity.0).unwrap();
        let health_sprite_height = assets.get(health_sprite_handle).unwrap().size().y as f32;
        bar_transform.translation.x = health_transform.translation.x;
        bar_transform.translation.y =
            health_transform.translation.y + health_sprite_height / 2. + 10.;
    }
}

fn display_current_health(
    bar_query: Query<&Healthbar>,
    mut bar_fill_query: Query<(&mut Transform, &Parent), With<HealthbarFill>>,
    health_query: Query<&Health>,
) {
    for (mut transform, bar_entity) in bar_fill_query.iter_mut() {
        let health_entity = bar_query.get(bar_entity.get()).unwrap().0;
        let health = health_query.get(health_entity).unwrap();
        transform.scale.x = health.percent();
    }
}

fn hide_full_hp_bar(
    mut bar_query: Query<(&Healthbar, &mut Visibility)>,
    health_query: Query<&Health>,
) {
    for (health_entity, mut visibility) in bar_query.iter_mut() {
        let health = health_query.get(health_entity.0).unwrap();
        if health.is_full() {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

fn healing(mut query: Query<(&mut Health, &HealingCurve, &mut HealingTimer)>, time: Res<Time>) {
    for (mut health, heal_curve, mut heal_timer) in query.iter_mut() {
        if !health.is_full() {
            heal_timer.0.tick(time.delta());
            health.current = health.max * heal_curve.0.ease(heal_timer.0.fraction());
        }
        //info!("Health Old: {}", health.current);
        //info!("Health Ease: {}", healing_curve.0.ease(health.percent()));
        //health.current = health.max
        //    * healing_curve
        //        .0
        //        .ease(health.percent() * time.delta_seconds());
        //health.current += 1. * time.delta_seconds();
        health.current = if health.current >= health.max {
            health.max
        } else {
            health.current
        };
        //info!("Health New: {}\n", health.current);
    }
}
