//! Stuffs that haven't been on the three main directories in the original template
pub mod tiled;
pub mod cam; // Named to be distinct from bevy::camera

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        tiled::plugin,
        cam::plugin,
    ));
}