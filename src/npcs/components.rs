use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

/// The Healthbar marker
#[derive(Component)]
pub struct Healthbar(pub Entity);

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max
        }
    }
    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component)]
pub struct HealingCurve(pub CubicSegment<Vec2>);

impl HealingCurve {
    pub fn new(control1: Vec2, control2: Vec2) -> Self {
        Self(CubicSegment::new_bezier(control1, control2))
    }
}

impl Default for HealingCurve {
    fn default() -> Self {
        Self::new(Vec2::new(0.25, 0.1), Vec2::new(0.25, 1.))
    }
}

#[derive(Bundle)]
pub struct HealthBundle {
    pub health: Health
}

impl HealthBundle {
    pub fn new(max: f32) -> Self {
        Self {
            health: Health::new(max)
        }
    }
}
