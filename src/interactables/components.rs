use avian2d::prelude::{Collider, CollidingEntities, Sensor};
use bevy::{prelude::*, utils::hashbrown::HashSet};

#[derive(Event)]
pub enum InteractableAction {
    UseOrEquip(Entity),
}
#[derive(Component)]
pub struct InteractorRange(pub f32);

impl Default for InteractorRange {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Component)]
pub struct InteractorSensor;

#[derive(Bundle)]
pub struct InteractorBundle {
    pub interactor_sensor: InteractorSensor,
    pub collider: Collider,
    pub sensor: Sensor,
    pub interactable_items: InteractableItems,
    pub spatial_bundle: SpatialBundle,
}

impl InteractorBundle {
    pub fn new(range: f32) -> Self {
        Self {
            interactor_sensor: InteractorSensor,
            collider: Collider::circle(range),
            sensor: Sensor,
            interactable_items: InteractableItems::default(),
            spatial_bundle: SpatialBundle::INHERITED_IDENTITY,
        }
    }
}

impl From<HashSet<Entity>> for InteractableItems {
    fn from(value: HashSet<Entity>) -> Self {
        let mut result = Self(vec![]);
        for i in value.iter() {
            result.0.push(*i);
        }
        result
    }
}
impl<'a> From<&'a HashSet<Entity>> for InteractableItems {
    fn from(value: &'a HashSet<Entity>) -> Self {
        let mut result = Self(vec![]);
        for i in value.iter() {
            result.0.push(*i);
        }
        result
    }
}
impl FromIterator<Entity> for InteractableItems {
    fn from_iter<I: IntoIterator<Item = Entity>>(value: I) -> Self {
        let mut result = Self(vec![]);
        for i in value {
            result.0.push(i);
        }
        result
    }
}

impl Default for InteractorBundle {
    fn default() -> Self {
        Self::new(100.)
    }
}

/// Items are sorted by distance to interactor entity
#[derive(Component, Reflect)]
pub struct InteractableItems(pub Vec<Entity>);

impl Default for InteractableItems {
    fn default() -> Self {
        Self(vec![])
    }
}
impl InteractableItems {
    pub fn closest_item(&self) -> Entity {
        self.0[0]
    }
    pub fn update(
        &mut self,
        player_equip_range: &InteractorRange,
        player_transform: &Transform,
        items: Vec<(Entity, &Transform)>,
    ) {
        let filtered_items: Vec<(Entity, &Transform)> = items
            .clone()
            .into_iter()
            .filter(|(_entity, transform)| {
                transform.translation.distance(player_transform.translation) < player_equip_range.0
            })
            .collect();
        let mut items_by_dist_to_player = filtered_items.clone();
        items_by_dist_to_player.sort_by(|(_entity0, transform0), (entity1, transform1)| {
            transform0
                .translation
                .distance(player_transform.translation)
                .partial_cmp(
                    &transform1
                        .translation
                        .distance(player_transform.translation),
                )
                .unwrap()
        });
        self.0 = items_by_dist_to_player
            .iter()
            .map(|(entity, _transform)| *entity)
            .collect();
    }
}

#[derive(Component)]
pub struct Interactable;
