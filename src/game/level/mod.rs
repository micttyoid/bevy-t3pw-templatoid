use bevy::prelude::*;

use crate::{
    asset_loader::GameplayAssets, audio::music, game::player::player, screens::Screen,
    utils::tiled as utils_tiled,
};

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, gameplay_assets: Res<GameplayAssets>) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            player(1.0, &gameplay_assets.character.clone()),
            (
                Name::new("Gameplay Music"),
                music(gameplay_assets.gameplay_bg_music.clone())
            ),
            utils_tiled::TiledMapBundle {
                tiled_map: utils_tiled::TiledMapHandle(gameplay_assets.map.clone()),
                ..default()
            }
        ],
    ));
}
