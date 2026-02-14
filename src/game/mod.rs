//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.
mod animation;
pub mod level;
mod movement;
pub mod player;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
    app.add_systems(Update, update_red);
}

/// Get red temporarily
/// Usage: insert it.
#[derive(Component)]
pub struct Red(Timer);

impl Default for Red {
    fn default() -> Self {
        Self {
            0: Timer::from_seconds(Self::DEFAULT_DURATION, TimerMode::Once),
        }
    }
}

impl Red {
    pub const DEFAULT_DURATION: f32 = 1.0;
    pub const N_BLINKS: usize = 6;
}

// It could be efficient if this goes more discrete, not over frames, but that's only for such type of color trasition
// You can also use smooth functions instead (for such color transition) and this works better.
fn update_red(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &mut Sprite, &mut Red)>,
) {
    let d = time.delta();
    for (entity, mut sprite, mut red) in query {
        let redness = (Red::DEFAULT_DURATION * 2.0 * (Red::N_BLINKS as f32) * red.0.elapsed_secs())
            .cos()
            .signum()
            / 4.0
            + 0.75; // 0.5 ~ 1.0
        if red.0.is_finished() {
            sprite.color = Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0));
            commands.entity(entity).remove::<Red>();
        } else {
            red.0.tick(d);
            sprite.color = Color::Srgba(Srgba::new(redness, 0.35, 0.35, 1.0));
        }
    }
}
