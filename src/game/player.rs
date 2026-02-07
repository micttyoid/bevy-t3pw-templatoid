//! Player-specific behavior.

use bevy::prelude::*;

use bevy_aseprite_ultra::prelude::{
    Animation, AnimationDirection, AnimationRepeat, AseAnimation, Aseprite,
};
use bevy_enhanced_input::prelude::*;

use crate::game::movement::Movement;

/// The player character.
pub fn player(max_speed: f32, sprite: &Handle<Aseprite>) -> impl Bundle {
    (
        Name::new("Player"),
        Player {
            walk_speed: max_speed,
            state: PlayerState::Idle,
            direction: PlayerDirection::Right,
        },
        AseAnimation {
            animation: Animation::tag("walk-up")
                .with_repeat(AnimationRepeat::Loop)
                .with_direction(AnimationDirection::Forward)
                .with_speed(2.0),
            aseprite: sprite.clone(),
        },
        Sprite::default(),
        Transform::from_xyz(0., 0., 100.),
        actions!(
            Player[(
                Action::<Movement>::new(),
                DeadZone::default(),
                SmoothNudge::default(),
                Scale::splat(128.0),
                Bindings::spawn((
                    Cardinal::wasd_keys(),
                    Cardinal::arrows(),
                    Axial::left_stick(),
                )),
            )]
        ),
    )
}

#[derive(Component, Debug)]
pub struct Player {
    pub walk_speed: f32,
    pub state: PlayerState,
    pub direction: PlayerDirection,
}

#[derive(Debug, PartialEq)]
pub enum PlayerState {
    Walk,
    Idle,
}
#[derive(Debug, PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}
