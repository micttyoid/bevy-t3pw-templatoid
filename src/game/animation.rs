//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use bevy::prelude::*;
use bevy_aseprite_ultra::{AsepriteUltraPlugin, prelude::AseAnimation};

use crate::game::player::{Player, PlayerDirection, PlayerState};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_plugins(AsepriteUltraPlugin);
    app.add_systems(Update, update_animation);
}

fn update_animation(mut anim_q: Query<(&mut AseAnimation, &Player)>) {
    for (mut animation, char) in anim_q.iter_mut() {
        match char.state {
            PlayerState::Idle => {
                animation.animation.play_loop("idle");
            }
            PlayerState::Walk => match char.direction {
                PlayerDirection::Up => {
                    animation.animation.play_loop("walk-up");
                }
                PlayerDirection::Down => {
                    animation.animation.play_loop("walk-down");
                }
                _ => {
                    animation.animation.play_loop("walk-right");
                }
            },
        }
    }
}
