use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};

/// The direction the entity is facing
#[derive(Component, Clone, Debug)]
pub enum Facing {
    Left,
    Right,
}

#[derive(Event)]
pub struct StateChange {
    pub state: GraphicsState,
    pub entity: Entity
}

pub enum AnimTimer {
    AnimTimer(Timer),
    AnimVelocityList(Vec<f32>)
}
#[derive(Component)]
pub struct AnimationList(pub Vec<StateAnimation>);

#[derive(Component)]
pub struct StateAnimation {
    pub indexes: (usize, usize),
    pub anim_timer: AnimTimer,
    pub trigger_state: GraphicsState
}

impl StateAnimation {
    pub fn new_timer(start: usize, finish: usize, trigger_state: GraphicsState, duration: u64) -> Self {
        Self {
            indexes: (start, finish),
            anim_timer: AnimTimer::AnimTimer(Timer::new(Duration::from_millis(duration), TimerMode::Repeating)),
            trigger_state
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
        self.indexes.1 - self.indexes.0
    }
    pub fn frame_from_percent(&self, percent: f32) -> usize {
        if !self.has_velocity_list() {
            self.indexes.0 + self.frame_range() * ((percent * 100.) as usize) / 100
        } else {
            panic!("Doesn't have a timer");
        }
    }
    pub fn has_velocity_list(&self) -> bool {
        match self.anim_timer {
            AnimTimer::AnimTimer(_) => false,
            AnimTimer::AnimVelocityList(_) => true
        }
    }
    //pub fn frame_from_velocity(&self, velocity: f32) -> usize {
    //}
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

#[derive(Component, Clone, Debug, PartialEq)]
pub enum GraphicsState {
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

#[derive(Bundle)]
pub struct PlayerGraphicsBundle {
    pub graphics_bundle: GraphicsBundle,
    pub state: GraphicsState,
    pub animation_list: AnimationList,
}

#[derive(Bundle)]
pub struct GraphicsBundle {
    pub sprite: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub facing: Facing,
}

impl GraphicsBundle {
    pub fn new(
        handle: Handle<Image>,
        mut texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        tile_size: UVec2,
        tile_columns: u32,
        tile_rows: u32,
        origin: Vec2,
    ) -> Self {
        let atlas_layout = TextureAtlasLayout::from_grid(tile_size, tile_columns, tile_rows, Some(UVec2::new(1,1)), None);
        let texture_atlas_layout = texture_atlas_layouts.add(atlas_layout);
        Self {
            sprite: SpriteBundle {
                texture: handle,
                transform: Transform::from_translation(origin.extend(0.)).with_scale(Vec3::splat(2.)),
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 1
            },
            facing: Facing::Right
        }
    }
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.sprite.sprite.anchor = anchor;
        self
    }
}
