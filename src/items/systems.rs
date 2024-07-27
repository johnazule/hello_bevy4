use avian2d::collision::{Collider, CollidingEntities};
use crate::Player;
use bevy::prelude::*;
use super::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemAction>()
            .add_systems(Update, (
                use_item,
                equip_item,
                equipped_item_follow_player,
                handle_item_actions
        ));
    }
}

fn use_item(item_query: Query<(&Item, &Transform)>) {
    
}

fn equip_item(
    mut commands: Commands,
    player_query: Query<(&CollidingEntities), (With<Player>)>,
    item_query: Query<(&Collider, Entity), (With<Item>, Without<Equipped>)>) {
    //let (player_collider, colliding_entities) = player_query.single();
    for (colliding_entities) in player_query.iter() {
        for (item_collider, item_entitiy) in item_query.iter() {
            if colliding_entities.0.contains(&item_entitiy) {
                info!("Equipped!!");
                commands.entity(item_entitiy).insert(Equipped);
            } 
        }
    }
}

fn equipped_item_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut item_query: Query<&mut Transform, (With<Equipped>, Without<Player>)>,
) {
    for player in player_query.iter() {
        for mut item in item_query.iter_mut() {
            item.translation = player.translation;
        }
    }
}

fn handle_item_actions(
    mut item_event_reader: EventReader<ItemAction>
) {
    for event in item_event_reader.read() {
        match event {
           ItemAction::Use => {
               info!("Yay");
           }
           ItemAction::Eat => {

           }
        }
    }
}
