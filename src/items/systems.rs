use super::prelude::*;
use crate::{Facing, Player};
use avian2d::{collision::CollidingEntities, prelude::PhysicsSet};
use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemAction>().add_systems(
            Update,
            (
                use_item,
                equip_item,
                equipped_item_follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
                handle_item_actions,
            ),
        );
    }
}

fn use_item(
    mut item_query: Query<
        (
            &mut Transform,
            &SwingDesc,
            &UseAccel,
            &mut UseTime,
            &mut InUse,
            Entity,
        ),
        (With<Item>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Item>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for player_transform in player_query.iter() {
        for (mut transform, swing_desc, use_accel, mut use_time, in_use, entity) in
            item_query.iter_mut()
        {
            if use_time.0.fraction() == 0. {
                //transform.rotation = Quat::from_rotation_z(swing_desc.start_angle);
            }
            use_time.0.tick(time.delta());
            if use_time.0.finished() {
                use_time.0.reset();
                transform.rotation = Quat::from_rotation_z(swing_desc.rest_angle);
                commands.entity(entity).remove::<InUse>();
            } else {
                //info!("Swing Range: {}", swing_desc.swing_range());
                transform.rotation = Quat::from_rotation_z(
                    swing_desc.start_angle_bounded()
                        + swing_desc.swing_range() * (use_accel.0.ease(use_time.0.fraction())),
                );
            }
            // Might cause an edge case where item doesn't begin swing from start_angle
            //if transform.rotation == Quat::from_rotation_z(swing_desc.end_angle_bounded()) {
            //    transform.rotation = Quat::from_rotation_z(swing_desc.rest_angle_bounded());
            //    commands.entity(entity).remove::<InUse>();
            //}
            //if transform.rotation == Quat::from_rotation_z(swing_desc.rest_angle_bounded()) {
            //    transform.rotation = Quat::from_rotation_z(swing_desc.start_angle_bounded());
            //}
            //let item_current_rotation = transform.rotation.angle_between(Quat::from_rotation_z(0.));
            //let use_percent = swing_desc.use_percent(item_current_rotation);
            //let new_angle = item_current_rotation + (use_accel.velc_function)(use_percent);
            //info!("Use % (calc): {}", use_percent);
            //info!("Use % (real): {}", in_use.use_percent);
            //transform.rotate_around(player_transform.translation, Quat::from_rotation_z(new_angle * time.delta_seconds()));
            //in_use.use_percent = new_angle * time.delta_seconds
        }
    }
}

fn equip_item(
    mut commands: Commands,
    player_query: Query<&CollidingEntities, With<Player>>,
    mut item_query: Query<
        (&mut Transform, Option<&SwingDesc>, Entity),
        (With<Item>, Without<Equipped>),
    >,
) {
    player_query.iter().for_each(|colliding_entities| {
        for (mut item_transform, swing_desc, item_entitiy) in item_query.iter_mut() {
            if colliding_entities.0.contains(&item_entitiy) {
                info!("Equipped!!");
                commands.entity(item_entitiy).insert(Equipped);
                if swing_desc.is_some() {
                    item_transform.rotation = Quat::from_rotation_z(swing_desc.unwrap().rest_angle);
                }
            }
        }
    });
}

fn equipped_item_follow_player(
    player_query: Query<(Entity, &Facing), With<Player>>,
    mut transform_parems: ParamSet<(
        TransformHelper,
        Query<(&mut Transform, &mut Facing), (With<Equipped>, Without<Player>)>,
    )>,
) {
    let player_result = player_query.get_single();
    match player_result {
        Ok((player_entity, player_facing)) => {
            let player_transform = transform_parems
                .p0()
                .compute_global_transform(player_entity)
                .unwrap();
            for (mut item_transform, mut item_facing) in transform_parems.p1().iter_mut() {
                item_transform.translation.x = player_transform.translation().x;
                item_transform.translation.y = player_transform.translation().y - 10.;
                *item_facing = player_facing.clone();
                //info!("Item Facing ({:?}) is Player Facing ({:?})", item_facing, player_facing);
            }
        }
        Err(_) => return,
    }
}

fn handle_item_actions(
    mut item_event_reader: EventReader<ItemAction>,
    mut commands: Commands,
    time: Res<Time>,
    mut item_query: Query<(Entity, &mut Transform, &SwingDesc), (With<Item>, With<Equipped>)>,
) {
    for event in item_event_reader.read() {
        let item = item_query.get_single_mut();
        if item.is_ok() {
            let (item_entity, mut item_transform, swing_desc) = item.unwrap();
            match event {
                ItemAction::Use => {
                    //info!("Yay");
                    commands.entity(item_entity).insert(InUse::default());
                }
                ItemAction::Eat => {}
                ItemAction::Start => {
                    item_transform.rotation = Quat::from_rotation_z(swing_desc.start_angle);
                }
                ItemAction::End => {
                    item_transform.rotation = Quat::from_rotation_z(swing_desc.end_angle);
                }
                ItemAction::Rest => {
                    item_transform.rotation = Quat::from_rotation_z(swing_desc.rest_angle);
                }
            }
        }
    }
}
