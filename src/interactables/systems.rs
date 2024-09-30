use avian2d::prelude::CollidingEntities;
use bevy::{math::NormedVectorSpace, prelude::*};
use bevy_ecs_ldtk::systems::process_ldtk_assets;

use crate::{
    process_player, Equipped, Interactable, InteractableAction, InteractableItems,
    InteractorBundle, InteractorRange, InteractorSensor, Item,
};

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InteractableItems>()
            .add_event::<InteractableAction>()
            .add_systems(
                Update,
                (
                    spawn_interaction_sensors
                        .after(process_player)
                        .after(process_ldtk_assets),
                    update_interactable_items,
                    handle_interaction_events,
                ),
            );
    }
}

pub fn interact_item() {
    info!("Interacting");
}

pub fn spawn_interaction_sensors(
    mut commands: Commands,
    interactor_query: Query<(Entity, &InteractorRange), Added<InteractorRange>>,
) {
    for (entity, interactor_range) in interactor_query.iter() {
        info!("Here!");
        //let interactor_sensor = commands
        //    .spawn(InteractorBundle::new(interactor_range.0))
        //    .id();
        //commands.entity(entity).add_child(interactor_sensor);
        commands.entity(entity).with_children(|child_builder| {
            child_builder.spawn(InteractorBundle::new(interactor_range.0));
        });
    }
}

pub fn update_interactable_items(
    mut interactor_sensor_query: Query<
        (&CollidingEntities, &mut InteractableItems, &GlobalTransform),
        With<InteractorSensor>,
    >,
    items_query: Query<&Transform, (Without<InteractorSensor>, With<Item>, With<Interactable>)>,
) {
    for (interactor_sensor, mut interactable_items, interactor_transform) in
        interactor_sensor_query.iter_mut()
    {
        interactable_items.0 = interactor_sensor
            .0
            .iter()
            .filter_map(|e| {
                if items_query.contains(*e) {
                    Some(*e)
                } else {
                    None
                }
            })
            .collect();
        //let debug_map: Vec<f32> = interactable_items
        //    .0
        //    .clone()
        //    .iter()
        //    .map(|e| {
        //        items_query
        //            .get(*e)
        //            .unwrap()
        //            .translation
        //            .distance(interactor_transform.compute_transform().translation)
        //    })
        //    .collect();
        //info!("Debug map: {:?}", debug_map);
        info!("Hello?");
        interactable_items.0.sort_by_key(|e| {
            items_query
                .get(*e)
                .unwrap()
                .translation
                .distance_squared(interactor_transform.translation()) as i32
        });
    }
}

pub fn handle_interaction_events(
    interactable_items_query: Query<&InteractableItems>,
    interactors_query: Query<&Children, With<InteractorRange>>,
    mut item_query: Query<&mut Equipped, With<Item>>,
    mut interactable_event_reader: EventReader<InteractableAction>,
    mut commands: Commands,
) {
    for event in interactable_event_reader.read() {
        match event {
            InteractableAction::UseOrEquip(interactor_entity) => {
                let interactor_result = interactors_query.get(*interactor_entity);
                if interactor_result.is_err() {
                    continue;
                }
                let interactor = interactor_result.unwrap();
                let interactable_items = interactor
                    .iter()
                    .filter_map(|child| interactable_items_query.get(*child).ok())
                    .take(1)
                    .next();
                if interactable_items.is_none() {
                    continue;
                }
                //let interactable_items = interactable_items_query
                //    .get(*interactable_item_entity.unwrap())
                //    .unwrap();
                commands
                    .entity(interactable_items.unwrap().0[0])
                    .insert(Equipped(*interactor_entity))
                    .remove::<Interactable>();
            }
        }
    }
}
