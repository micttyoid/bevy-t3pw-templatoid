//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::{math::VectorSpace, prelude::*};
use rand::prelude::*;
use std::ops::Add;
use std::time::Duration;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    demo::{movement::MovementController, player::PlayerAssets},
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            update_animation_timer.in_set(AppSystems::TickTimers),
            (
                update_animation_movement,
                update_animation_atlas,
                trigger_step_sound_effect,
            )
                .chain()
                .in_set(AppSystems::Update),
        )
            .in_set(PausableSystems),
    );
}

/// Update the animation timer.
fn update_animation_timer(time: Res<Time>, mut query: Query<&mut PlayerAnimation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

/// Update the sprite direction and animation state (idling/walking).
fn update_animation_movement(
    mut player_query: Query<(&MovementController, &mut Sprite, &mut PlayerAnimation)>,
) {
    for (controller, mut sprite, mut animation) in &mut player_query {
        use Direction::*;
        use PlayerAnimationState::*;
        let animation_state = if controller.intent == Nothing {
            Idling(animation.get_direction()) // keep the early direction
        } else {
            match controller.intent {
                Left | UpLeft | DownLeft => {
                    sprite.flip_x = true;
                }
                _ => {
                    sprite.flip_x = false;
                }
            }
            Walking(controller.intent.clone())
        };
        animation.update_state(animation_state);
    }
}

/// Update the texture atlas to reflect changes in the animation.
fn update_animation_atlas(mut query: Query<(&PlayerAnimation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };
        if animation.changed() {
            atlas.index = animation.get_atlas_index();
        }
    }
}

/// If the player is moving, play a step sound effect synchronized with the
/// animation.
fn trigger_step_sound_effect(
    mut commands: Commands,
    player_assets: If<Res<PlayerAssets>>,
    mut step_query: Query<&PlayerAnimation>,
) {
    for animation in &mut step_query {
        if animation.is_walking() && animation.changed() && animation.frame == 1 {
            let rng = &mut rand::rng();
            let random_step = player_assets.steps.choose(rng).unwrap().clone();
            commands.spawn(sound_effect(random_step));
        }
    }
}

/// Component that tracks player's animation state.
/// It is tightly bound to the texture atlas we use.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerAnimation {
    timer: Timer,
    frame: usize,
    state: PlayerAnimationState,
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum PlayerAnimationState {
    Idling(Direction),
    Walking(Direction),
}

#[derive(Clone, Debug, Reflect, PartialEq, Default)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Nothing, // ex. Vec2::ZERO
}

impl Direction {
    pub fn get_vec2(&self) -> Vec2 {
        use Direction::*;
        match self {
            Nothing => Vec2::ZERO,
            Up => Vec2::new(0., 1.),
            Down => Vec2::new(0., -1.),
            Right => Vec2::new(1., 0.),
            Left => Vec2::new(-1., 0.),
            UpRight => Vec2::new(1.0, 1.0).normalize(),
            UpLeft => Vec2::new(-1.0, 1.0).normalize(),
            DownRight => Vec2::new(1.0, -1.0).normalize(),
            DownLeft => Vec2::new(-1.0, -1.0).normalize(),
        }
    }
}

impl PlayerAnimation {
    /// The number of idle frames.
    const IDLE_FRAMES: usize = 3;
    /// The duration of each idle frame.
    const IDLE_INTERVAL: Duration = Duration::from_millis(500);
    /// The number of walking frames.
    const WALKING_FRAMES: usize = 4;
    /// The duration of each walking frame.
    const WALKING_INTERVAL: Duration = Duration::from_millis(100);

    fn is_walking(&self) -> bool {
        match &self.state {
            PlayerAnimationState::Walking(_) => true,
            _ => false,
        }
    }

    fn get_direction(&self) -> Direction {
        use PlayerAnimationState::*;
        match &self.state {
            Idling(dir) | Walking(dir) => dir.clone(),
        }
    }

    fn idling(dir: Direction) -> Self {
        Self {
            timer: Timer::new(Self::IDLE_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Idling(dir),
        }
    }

    fn walking(dir: Direction) -> Self {
        Self {
            timer: Timer::new(Self::WALKING_INTERVAL, TimerMode::Repeating),
            frame: 0,
            state: PlayerAnimationState::Walking(dir),
        }
    }

    pub fn new() -> Self {
        Self::idling(Direction::Down)
    }

    /// Update animation timers.
    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if !self.timer.is_finished() {
            return;
        }
        self.frame = (self.frame + 1)
            % match &self.state {
                PlayerAnimationState::Idling(_) => Self::IDLE_FRAMES,
                PlayerAnimationState::Walking(_) => Self::WALKING_FRAMES,
            };
    }

    /*
    pub fn is_equal_state_but_direction(&self, state: &PlayerAnimationState) -> bool {
        use PlayerAnimationState::*;
        match (&self.state, state) {
            (Idling(_), Walking(_)) | (Walking(_), Idling(_)) => false,
            (Walking(_), Walking(_)) | (Idling(_), Idling(_)) => true,
        }
    }
    */

    /// Update animation state if it changes.
    pub fn update_state(&mut self, state: PlayerAnimationState) {
        use PlayerAnimationState::*;
        if self.state != state {
            match &state {
                PlayerAnimationState::Idling(dir) => *self = Self::idling(dir.clone()),
                PlayerAnimationState::Walking(dir) => *self = Self::walking(dir.clone()),
            }
        }
    }

    /// Whether animation changed this tick.
    pub fn changed(&self) -> bool {
        self.timer.is_finished()
    }

    /// Return sprite index in the atlas.
    pub fn get_atlas_index(&self) -> usize {
        use Direction::*;
        match &self.state {
            PlayerAnimationState::Idling(dir) => match dir {
                Up => 20 + self.frame,
                Down => 0 + self.frame,
                Right | Left => 10 + self.frame,
                UpRight | UpLeft => 15 + self.frame,
                DownRight | DownLeft => 5 + self.frame,
                _ => 8 + self.frame,
            },
            PlayerAnimationState::Walking(dir) => match dir {
                Up => 45 + self.frame,
                Down => 25 + self.frame,
                Right | Left => 35 + self.frame,
                UpRight | UpLeft => 40 + self.frame,
                DownRight | DownLeft => 30 + self.frame,
                _ => 25 + self.frame,
            },
        }
    }
}
