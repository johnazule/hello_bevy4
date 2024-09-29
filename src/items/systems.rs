use std::{f32::consts::PI, ops::Deref};

use super::prelude::*;
use crate::{
    process_player, Facing, GraphicsBundle, Interactable, InteractableItems, InteractorRange,
    Player,
};
use avian2d::{collision::CollidingEntities, prelude::PhysicsSet};
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::EntityInstance;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemAction>().add_systems(
            Update,
            (
                //update_item_resources,
                process_item,
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

//fn update_item_resources(
//    mut items_in_player_equip_range: ResMut<ItemsInInteractRange>,
//    player_query: Query<(&EquipRange, &Transform), (With<Player>, Without<Item>)>,
//    item_query: Query<(Entity, &Transform), (With<Item>, Without<Player>)>,
//) {
//    let Ok((player_equip_range, player_transform)) = player_query.get_single() else {
//        panic!("More than 1 player");
//    };
//    items_in_player_equip_range.update(
//        player_equip_range,
//        player_transform,
//        item_query.iter().collect::<Vec<(Entity, &Transform)>>(),
//    );
//}

fn process_item(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, entity_instance, transform) in new_entity_instances.iter() {
        match entity_instance.identifier.as_str() {
            "Carrot" => {
                commands.entity(entity).insert((
                    Name::new("Carrot Sword"),
                    GraphicsBundle::new(
                        asset_server.load("sprites/carrot.png"),
                        &mut texture_atlas_layouts,
                        UVec2::new(5, 30),
                        1,
                        1,
                        Vec2::new(100., 0.),
                    )
                    .with_anchor(Anchor::BottomRight)
                    .with_z_index(10.)
                    .with_transform(*transform),
                    ItemBundle::default()
                        //.with_position(50.,-50.)
                        .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
                        .with_use_time(250)
                        .with_swing_desc(4. * PI / 3., PI / 6., 4. * PI / 3.),
                    Interactable,
                ));
            }
            "Strawberry" => {
                commands.entity(entity).insert((
                    Name::new("Strawberry Dagger"),
                    GraphicsBundle::new(
                        asset_server.load("sprites/strawberry.png"),
                        &mut texture_atlas_layouts,
                        UVec2::new(3, 6),
                        1,
                        1,
                        Vec2::new(-100., 0.),
                    )
                    .with_anchor(Anchor::BottomRight)
                    .with_z_index(10.)
                    .with_transform(*transform),
                    ItemBundle::default()
                        //.with_position(50.,-50.)
                        .with_use_accel(CubicSegment::new_bezier((0.25, 0.1), (0.25, 1.)))
                        .with_use_time(180)
                        .with_swing_desc(PI / 6., PI / 12., PI / 6.),
                    Interactable,
                ));
            }
            _ => {}
        };
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
    equiper_query: Query<(&CollidingEntities, Entity, &InteractorRange), Without<Item>>,
    mut item_query: Query<
        (&mut Transform, Option<&SwingDesc>, Entity),
        (With<Item>, Without<Equipped>),
    >,
) {
    //equiper_query
    //    .iter()
    //    .for_each(|(colliding_entities, equiper_entity, equip_range)| {
    //        for (mut item_transform, swing_desc, item_entitiy) in item_query.iter_mut() {
    //            if colliding_entities.0.contains(&item_entitiy) {
    //                info!("Equipped!!");
    //                commands
    //                    .entity(item_entitiy)
    //                    .insert(Equipped(equiper_entity));
    //                if swing_desc.is_some() {
    //                    item_transform.rotation =
    //                        Quat::from_rotation_z(swing_desc.unwrap().rest_angle);
    //                }
    //            }
    //        }
    //    });
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

fn interact_item() {
    info!("Interacting");
}

fn handle_item_actions(
    mut item_event_reader: EventReader<ItemAction>,
    mut commands: Commands,
    time: Res<Time>,
    player_query: Query<(Entity, &Children), With<Player>>,
    interactable_items_query: Query<&InteractableItems>,
    mut item_query: Query<(Entity, &mut Transform, &SwingDesc, Option<&Equipped>), With<Item>>,
) {
    //for (player_entity, player_children) in player_query.iter() {
    //    let player_interactable_items_result = player_children
    //        .iter()
    //        .filter_map(|child| interactable_items_query.get(*child).ok())
    //        .take(1)
    //        .next();
    //    //let mut player_interactable_items_result;
    //    //for child in player_children {
    //    //    if interactable_items_query.contains(*child) {
    //    //        player_interactable_items_result = interactable_items_query.get(*child);
    //    //    }
    //    //}
    //    if player_interactable_items_result.is_none() {
    //        info!("Here hehe");
    //        continue;
    //    }
    //    let player_interactable_items = player_interactable_items_result.unwrap();
    //let player_interactable_items = interactable_items_query.get(player_children)
    //let player_interactable_items_entities_query = player_children
    //    .iter()
    //    .filter(|player_children| interactable_items_query.get(**player_children).is_ok())
    //    .take(1)
    //    .next();
    //if !player_interactable_items_entities_query.is_some() {
    //    info!("Here hehe");
    //    return;
    //}
    //let player_interactable_items_entity = player_interactable_items_entities_query.unwrap();
    //let player_interactable_item_closest = item_query
    //    .get(*player_interactable_items_entity)
    //    .unwrap()
    //    .0
    //    .clone();
    for event in item_event_reader.read() {
        for (mut item_entity, mut item_transform, swing_desc, is_equipped) in item_query.iter_mut()
        {
            match event {
                //ItemAction::Interact(event_item_entity) => {
                //    //for (mut item_entity, mut item_transform, swing_desc, is_equipped) in item_query
                //    //    .iter_mut()
                //    //    //.filter(|item| player_interactable_items.0.contains(&item.0))
                //    //    .filter(|item| player_interactable_items.0[0] == item.0)
                //    //{
                //    //    let interact_item_id = commands.register_one_shot_system(interact_item);
                //    //    commands.run_system(interact_item_id);
                //    //}
                //}
                ItemAction::Use(event_item_entity) => {
                    commands.entity(*event_item_entity).insert(InUse::default());
                }
                ItemAction::Eat(event_item_entity) => {}
                ItemAction::Start(event_item_entity) => {
                    *item_transform.rotation =
                        *Quat::from_rotation_z(swing_desc.start_angle).deref();
                }
                ItemAction::End(event_item_entity) => {
                    *item_transform.rotation = *Quat::from_rotation_z(swing_desc.end_angle).deref();
                }
                ItemAction::Rest(event_item_entity) => {
                    *item_transform.rotation =
                        *Quat::from_rotation_z(swing_desc.rest_angle).deref();
                }
            }
        }
        //}
    }
}
