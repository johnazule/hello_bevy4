use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::LdtkIntCell;
use bevy_light_2d::light::PointLight2d;

/// The direction the entity is facing
#[derive(Component, Clone, Debug, Default)]
pub enum Facing {
    #[default]
    Left,
    Right,
}

#[derive(Event)]
pub struct StateChange {
    pub state: GraphicsState,
    pub entity: Entity,
}

#[derive(Debug)]
pub enum AnimTimer {
    AnimTimer(Timer),
    AnimVelocityList(Vec<f32>),
}

impl AnimTimer {
    pub fn timer_fraction(&self) -> f32 {
        match self {
            AnimTimer::AnimTimer(timer) => timer.fraction(),
            AnimTimer::AnimVelocityList(_velocity_list) => {
                panic!("Doesn't have a timer");
            }
        }
    }
    pub fn tick_timer(&mut self, delta: Duration) {
        match self {
            AnimTimer::AnimTimer(timer) => {
                timer.tick(delta);
            }
            AnimTimer::AnimVelocityList(_velocity_list) => {
                panic!("Doesn't have a timer");
            }
        }
    }
    pub fn velocity_fraction(&self, velocity: f32) -> f32 {
        match self {
            AnimTimer::AnimTimer(_timer) => {
                panic!("Doesn't have velocity lists");
            }
            AnimTimer::AnimVelocityList(velocity_thresholds) => {
                let mut threshold_index: f32 = 0.;
                //velocity_thresholds.clone().iter().filter(|threshold| threshold < )
                for (i, velocity_threshold) in velocity_thresholds.iter().enumerate() {
                    if velocity.abs() < velocity_threshold.abs() {
                        break;
                    }
                    threshold_index = i as f32;
                }
                threshold_index / velocity_thresholds.len() as f32
            }
        }
    }
}

#[derive(Component, Default)]
pub struct AnimationList(pub HashMap<GraphicsState, StateAnimation>);

//#[derive(Component)]
#[derive(Debug)]
pub struct StateAnimation {
    pub indexes: (usize, usize),
    pub anim_timer: AnimTimer,
    //pub trigger_state: GraphicsState
}

impl StateAnimation {
    pub fn new_timer(
        start: usize,
        finish: usize,
        /* trigger_state: GraphicsState,*/ duration: u64,
    ) -> Self {
        Self {
            indexes: (start, finish),
            anim_timer: AnimTimer::AnimTimer(Timer::new(
                Duration::from_millis(duration),
                TimerMode::Repeating,
            )),
            //trigger_state
        }
    }
    pub fn new_velocity_list(start: usize, finish: usize, velocity_list: Vec<f32>) -> Self {
        Self {
            indexes: (start, finish),
            anim_timer: AnimTimer::AnimVelocityList(velocity_list),
        }
    }
    //pub fn new_timer(start: usize, finish: usize, trigger_state: GraphicsState, duration: u64) -> Self {
    //    Self {
    //        indexes: (start, finish),
    //        anim_timer: AnimTimer::AnimTimer(Timer::new(Duration::from_millis(duration), TimerMode::Repeating)),
    //        trigger_state
    //    }
    //}
    pub fn frame_range(&self) -> usize {
        self.indexes.0.abs_diff(self.indexes.1)
    }
    pub fn frame_from_percent(&self) -> usize {
        if !self.has_velocity_list() {
            self.indexes.0
                + self.frame_range() * ((self.anim_timer.timer_fraction() * 100.) as usize) / 100
        } else {
            panic!("Doesn't have a timer");
        }
    }
    pub fn has_velocity_list(&self) -> bool {
        match self.anim_timer {
            AnimTimer::AnimTimer(_) => false,
            AnimTimer::AnimVelocityList(_) => true,
        }
    }
    pub fn frame_from_velocity(&self, velocity: f32) -> usize {
        if self.has_velocity_list() {
            let frame: usize;
            if self.indexes.0 < self.indexes.1 {
                frame = self.indexes.0
                    + self.frame_range()
                        * ((self.anim_timer.velocity_fraction(velocity) * 100.) as usize)
                        / 100;
            } else {
                frame = self.indexes.0
                    - self.frame_range()
                        * ((self.anim_timer.velocity_fraction(velocity) * 100.) as usize)
                        / 100;
            }
            //info!("Velocity:\t{}", velocity);
            //info!("Fram:\t\t{}", frame);
            frame
        } else {
            panic!("Doesn't have a velocity list");
        }
    }
}

//#[derive(Component)]
//pub struct StateAnimationSpeed {
//    pub idle_timer: Timer,
//    pub running_timer: Timer,
//    pub jump_velocity: Vec<f32>,
//    pub falling_velocity: Vec<f32>,
//}
//impl StateAnimationSpeed {
//    fn new(idle_duration: u64, run_duration: u64) -> Self {
//        Self {
//            idle_timer: Timer::new(Duration::from_millis(idle_duration), TimerMode::Repeating),
//            running_timer: Timer::new(Duration::from_millis(run_duration), TimerMode::Repeating),
//            jump_velocity: vec![],
//            falling_velocity: vec![],
//        }
//    }
//    fn with_jump_velocities
//}

#[derive(Component, Clone, Debug, PartialEq, Hash, Eq, Default)]
pub enum GraphicsState {
    #[default]
    Idle,
    Running,
    Jumping,
    Falling,
}

impl GraphicsState {
    pub fn is_state(&self, graphics_state: &GraphicsState) -> bool {
        self == graphics_state
    }
    pub fn is_not_state(&self, graphics_state: &GraphicsState) -> bool {
        self != graphics_state
    }
}

#[derive(Bundle, Default)]
pub struct PlayerGraphicsBundle {
    //pub graphics_bundle: GraphicsBundle,
    pub facing: Facing,
    pub state: GraphicsState,
    pub animation_list: AnimationList,
}

#[derive(Bundle, Default)]
pub struct GraphicsBundle {
    pub sprite: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub facing: Facing,
}

impl GraphicsBundle {
    pub fn new(
        handle: Handle<Image>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_size: UVec2,
        tile_columns: u32,
        tile_rows: u32,
        origin: Vec2,
    ) -> Self {
        let atlas_layout = TextureAtlasLayout::from_grid(
            tile_size,
            tile_columns,
            tile_rows,
            Some(UVec2::new(1, 1)),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(atlas_layout);
        Self {
            sprite: SpriteBundle {
                texture: handle,
                transform: Transform::from_translation(origin.extend(0.))
                    .with_scale(Vec3::splat(2.)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            },
            facing: Facing::Right,
        }
    }
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.sprite.sprite.anchor = anchor;
        self
    }
    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.sprite.transform.translation.z = z_index;
        self
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite.transform = transform;
        self
    }
}

#[derive(Bundle, Clone, LdtkIntCell)]
pub struct GreenLightingBundle {
    pub light2d: PointLight2d,
}
impl Default for GreenLightingBundle {
    fn default() -> Self {
        Self {
            light2d: PointLight2d {
                radius: 150.0,
                intensity: 0.5,
                cast_shadows: true,
                falloff: 3.5,
                color: Color::linear_rgb(0., 1., 0.2),
            },
        }
    }
}
