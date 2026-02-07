//! Handle player input and translate it into movement through a character
//! controller. A character controller is the collection of systems that govern
//! the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set [`MovementController`] intent based on directional keyboard input.
//!   This is done in the `player` module, as it is specific to the player
//!   character.
//! - Apply movement based on [`MovementController`] intent and maximum speed.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is limited for demonstration
//! purposes. If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
//use avian2d::prelude::*;
use crate::game::player::{Player, PlayerDirection, PlayerState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<Player>()
        .add_observer(apply_movement);
}

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
pub struct Movement;

fn apply_movement(movement: On<Fire<Movement>>, mut query: Query<(&mut Transform, &mut Player)>) {
    let (mut transform, mut player) = query.get_mut(movement.context).unwrap();
    let direction = movement.value.normalize();
    if direction != Vec2::ZERO {
        player.state = PlayerState::Walk;
        transform.translation.x += player.walk_speed * direction.x;
        transform.translation.y += player.walk_speed * direction.y;
        if direction.y.abs() > direction.x.abs() {
            player.direction = if direction.y > 0.0 {
                PlayerDirection::Up
            } else {
                PlayerDirection::Down
            };
        } else {
            if direction.x > 0.0 {
                player.direction = PlayerDirection::Right;
                transform.scale.x = 1.0;
            } else {
                player.direction = PlayerDirection::Left;
                transform.scale.x = -1.0;
            };
        }
    } else {
        player.state = PlayerState::Idle;
    }
    info!("Moved {:?}", movement.value);
}
