use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

/// The Healthbar marker
#[derive(Component)]
pub struct Healthbar(pub Entity);

#[derive(Component)]
pub struct HealthbarBorder;

#[derive(Component)]
pub struct HealthbarFill;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
    pub fn is_full(&self) -> bool {
        self.current == self.max
    }
}

#[derive(Component)]
pub struct HealingCurve(pub CubicSegment<Vec2>);

#[derive(Component)]
pub struct HealingTimer(pub Timer);

impl HealingTimer {
    fn new(duration: u64) -> Self {
        Self(Timer::new(Duration::from_millis(duration), TimerMode::Once))
    }
}

impl HealingCurve {
    pub fn new(control1: Vec2, control2: Vec2) -> Self {
        Self(CubicSegment::new_bezier(control1, control2))
    }
}

impl Default for HealingCurve {
    fn default() -> Self {
        Self::new(Vec2::new(0., 0.), Vec2::new(1., 1.))
    }
}

#[derive(Bundle)]
pub struct HealthBundle {
    pub health: Health,
    pub healing_curve: HealingCurve,
    pub healing_timer: HealingTimer,
}

impl HealthBundle {
    pub fn new(max: f32, heal_curve: [[f32; 2]; 2], heal_curve_duration: u64) -> Self {
        Self {
            health: Health::new(max),
            healing_curve: HealingCurve(CubicSegment::new_bezier(
                Vec2::from_array(heal_curve[0]),
                Vec2::from_array(heal_curve[1]),
            )),
            healing_timer: HealingTimer::new(heal_curve_duration),
        }
    }
    pub fn with_current_health(mut self, health: f32) -> Self {
        self.health.current = health;
        self
    }
}
