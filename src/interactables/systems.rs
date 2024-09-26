use bevy::prelude::*;

use crate::{InteractorBundle, InteractorRange};

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_interaction_sensors);
    }
}

pub fn interact_item() {
    info!("Interacting");
}

pub fn spawn_interaction_sensors(
    mut commands: Commands,
    interactor_query: Query<(Entity, &InteractorRange)>,
) {
    for (entity, interactor_range) in interactor_query.iter() {
        commands.entity(entity).with_children(|child_builder| {
            child_builder.spawn(InteractorBundle::new(interactor_range.0));
        });
    }
}
